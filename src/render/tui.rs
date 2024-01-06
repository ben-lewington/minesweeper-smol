use crate::types::{Board, Cell};

use std::io::Stdout;

use crossterm::{
    cursor,
    style::{self, Stylize},
    QueueableCommand,
};

const MINE: char = '@';
const COVERED: char = '.';
const FLAGGED: char = 'F';
const COL_STRIDE: usize = 3;

const fn braces(selected: bool) -> (char, char) {
    if selected {
        return ('[', ']');
    }
    (' ', ' ')
}

const BOX_TOP_LEFT: char = '\u{250F}';
const BOX_HORIZONTAL: char = '\u{2501}';
const BOX_TOP_RIGHT: char = '\u{2513}';
const BOX_VERTICAL: char = '\u{2503}';
const BOX_BOTTOM_LEFT: char = '\u{2517}';
const BOX_BOTTOM_RIGHT: char = '\u{251B}';

impl Cell {
    fn render_cell(&self, f: &mut Stdout) -> std::io::Result<()> {
        match (self.covered(), self.flagged()) {
            (true, true) => {
                f.queue(style::Print(FLAGGED))?;
            }
            (true, false) => {
                f.queue(style::Print(COVERED))?;
            }
            (false, _) => {
                if self.mine() {
                    f.queue(style::PrintStyledContent(MINE.with(style::Color::Red)))?;
                } else if self.neighbours() == 0 {
                    f.queue(style::Print(' '))?;
                } else {
                    f.queue(style::Print(self.neighbours()))?;
                }
            }
        };

        Ok(())
    }
}

impl<const C: usize, const N: usize> Board<C, N> {
    fn render_board_cell(&self, idx: usize, stdout: &mut Stdout) -> std::io::Result<()> {
        let (bl, br) = braces(idx == self.cursor as usize);
        stdout.queue(style::Print(bl))?;
        self.cells[idx].render_cell(stdout)?;
        stdout.queue(style::Print(br))?;

        Ok(())
    }

    #[cfg(debug_assertions)]
    fn render_debug_info(&self, stdout: &mut Stdout) -> std::io::Result<()> {
        stdout.queue(cursor::MoveDown(1))?;
        let output = format!(
            "({}, {}): {:?}, {} neighbouring mines",
            self.cursor as usize / C,
            self.cursor as usize % C,
            self.cells[self.cursor as usize],
            self.cells[self.cursor as usize].neighbours(),
        );
        stdout
            .queue(style::Print(&output))?
            .queue(cursor::MoveDown(1))?
            .queue(cursor::MoveLeft(output.len() as u16))?;

        Ok(())
    }

    pub fn render(
        &self,
        (c_0, r_0): (u16, u16),
        stdout: &mut Stdout,
    ) -> std::io::Result<()> {
        let mut r_off = 0;
        stdout.queue(cursor::MoveTo(c_0, r_0 + r_off))?;
        stdout.queue(style::Print(format!(
            "{}{}{}",
            BOX_TOP_LEFT,
            (0..C * COL_STRIDE + 2).map(|_| BOX_HORIZONTAL).collect::<String>(),
            BOX_TOP_RIGHT,
        )))?;

        r_off += 1;

        stdout
            .queue(cursor::MoveTo(c_0, r_0 + r_off))?
            .queue(style::Print(BOX_VERTICAL))?
            .queue(cursor::MoveRight(C as u16 * COL_STRIDE as u16 + 2))?
            .queue(style::Print(BOX_VERTICAL))?;

        r_off += 1;

        stdout.queue(cursor::MoveTo(c_0, r_0 + r_off))?;

        for idx in 0..N {
            if idx % C == 0 {


                stdout.queue(style::Print(format!("{} ", BOX_VERTICAL)))?;
            }
            self.render_board_cell(idx, stdout)?;
            if idx % C == C - 1 {
                stdout.queue(style::Print(format!(" {}", BOX_VERTICAL)))?;
                r_off += 1;
                stdout.queue(cursor::MoveTo(c_0, r_0 + r_off))?;
            }
        }

        stdout
            .queue(style::Print(BOX_VERTICAL))?
            .queue(cursor::MoveRight(C as u16 * COL_STRIDE as u16 + 2))?
            .queue(style::Print(BOX_VERTICAL))?;

        r_off += 1;

        stdout
            .queue(cursor::MoveTo(c_0, r_0 + r_off))?
            .queue(style::Print(format!(
                "{}{}{}",
                BOX_BOTTOM_LEFT,
                (0..C * 3 + 2).map(|_| BOX_HORIZONTAL).collect::<String>(),
                BOX_BOTTOM_RIGHT,
            )))?;

        r_off += 1;

        stdout.queue(cursor::MoveTo(c_0, r_0 + r_off))?;

        #[cfg(debug_assertions)]
        self.render_debug_info(stdout)?;

        Ok(())
    }
}
