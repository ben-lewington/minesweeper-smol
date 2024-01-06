use minesweeper::types::{Board, Cell};

use std::io::{stdout, Write};

use crossterm::{cursor, event, execute, queue, terminal, QueueableCommand};

const COL: usize = 20;
const ROW: usize = 20;
const N: usize = COL * ROW;

const COL_STRIDE: usize = 3;

fn main() -> std::io::Result<()> {
    let pcent_mines = 0.125_f32;

    let mut rng = rand::thread_rng();
    let mut board = Board::<COL, N>::new_from_rng(&mut rng, pcent_mines);
    let mut stdout = stdout();

    let (mut col, mut row) = terminal::size()?;

    let topleft = |c: u16, r: u16| (
        c / 2 - COL as u16 * COL_STRIDE as u16 / 2,
        r / 2 - ROW as u16 / 2 - 1,
    );
    let (c_0, r_0) = topleft(col, row);

    terminal::enable_raw_mode()?;

    queue! {
        stdout,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
    }?;

    board.render((c_0, r_0), &mut stdout)?;

    stdout
        .queue(cursor::MoveTo(col, row))?
        .queue(cursor::DisableBlinking)?;

    stdout.flush()?;

    loop {
        use event::{Event, KeyCode, KeyEvent, KeyModifiers};

        match crossterm::event::read()? {
            Event::Key(KeyEvent {
                code,
                modifiers,
                kind: _,
                state: _,
            }) => match code {
                KeyCode::Char('h') | event::KeyCode::Left => {
                    let (r, c) = (
                        board.cursor / (COL as isize),
                        (board.cursor - 1).rem_euclid(COL as isize),
                    );
                    board.cursor = r * (COL as isize) + c;
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    board.cursor = (board.cursor + COL as isize).rem_euclid(N as isize)
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    board.cursor = (board.cursor - COL as isize).rem_euclid(N as isize)
                }
                KeyCode::Char('l') | KeyCode::Right => {
                    let (r, c) = (
                        board.cursor / (COL as isize),
                        (board.cursor + 1).rem_euclid(COL as isize),
                    );
                    board.cursor = r * (COL as isize) + c;
                }
                KeyCode::Char(' ') | KeyCode::Char('f') => {
                    board.cells[board.cursor as usize] ^= Cell::FLAGGED;
                }
                KeyCode::Enter => {
                    if board.cells[board.cursor as usize].mine() {
                        break;
                    } else if board.cells[board.cursor as usize].neighbours() > 0 {
                        board.cells[board.cursor as usize] ^= Cell::COVERED;
                    } else {
                        board.uncover_blank_neighbours(board.cursor as usize);
                    }
                }
                KeyCode::Char('c') if modifiers == modifiers | KeyModifiers::CONTROL => {
                    break;
                }
                #[cfg(debug_assertions)]
                KeyCode::Char('C') if modifiers == modifiers | KeyModifiers::SHIFT => {
                    for i in 0..N {
                        board.cells[i] ^= Cell::COVERED;
                    }
                }
                _ => {}
            }
            Event::Mouse(_) => {}
            _ => {}
        }

        (col, row) = terminal::size()?;
        let (c_0, r_0) = topleft(col, row);

        let _ = queue! {
            stdout,
            terminal::Clear(terminal::ClearType::All),
        };

        board.render((c_0, r_0), &mut stdout)?;

        #[cfg(debug_assertions)]
        {
            stdout
                .queue(cursor::MoveToColumn(0))?
                .queue(cursor::MoveDown(1))?;
        }

        stdout
            .queue(cursor::MoveTo(col, row))?
            .queue(cursor::DisableBlinking)?
            .queue(cursor::Hide)?;

        stdout.flush()?;
    }

    execute! {
        stdout,
        terminal::LeaveAlternateScreen,
        cursor::Show,
    }?;

    terminal::disable_raw_mode()?;
    Ok(())
}
