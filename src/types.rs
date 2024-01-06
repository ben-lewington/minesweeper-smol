pub struct Board<const C: usize, const N: usize> {
    pub cells: [Cell; N],
    pub cursor: isize,
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Cell: u8 {
        const MINE = 1 << 0;
        const COVERED = 1 << 1;
        const FLAGGED = 1 << 2;
        // NEIGHBOURS: u5
    }
}

impl<const C: usize, const N: usize> Board<C, N> {
    pub fn new_from_rng(rng: &mut impl rand::Rng, pcent_mines: f32) -> Self {
        let mut cells = [Cell::COVERED; N];

        for i in 0..N {
            let (r, c) = (i / C, i % C);
            let roll = rng.gen_range(0..N);
            if (roll as f32 / N as f32) < pcent_mines {
                cells[i] ^= Cell::MINE;

                if r > 0 {
                    if c > 0 {
                        cells[i - C - 1].inc_neighbours();
                    }
                    cells[i - C].inc_neighbours();
                    if c < C - 1 {
                        cells[i - C + 1].inc_neighbours();
                    }
                }
                if c > 0 {
                    cells[i - 1].inc_neighbours();
                }
                if c < C - 1 {
                    cells[i + 1].inc_neighbours();
                }
                if r < (N / C) - 1 {
                    if c > 0 {
                        cells[i + C - 1].inc_neighbours();
                    }
                    cells[i + C].inc_neighbours();
                    if c < C - 1 {
                        cells[i + C + 1].inc_neighbours();
                    }
                }
            }
        }

        Self { cells, cursor: 0 }
    }

    pub fn uncover_blank_neighbours(&mut self, idx: usize) {
        let cell = &mut self.cells[idx];
        if !cell.covered() || cell.mine() || cell.neighbours() > 0 { return; }
        *cell ^= Cell::COVERED;
        let (r, c) = (idx / C, idx % C);
        if r > 0 {
            let adj_cell = &mut self.cells[idx - C];
            if adj_cell.neighbours() > 0 && !adj_cell.mine() {
                if adj_cell.covered() {
                    *adj_cell ^= Cell::COVERED;
                }
            } else {
                self.uncover_blank_neighbours(idx - C);
            }
        }
        if c > 0 {
            let adj_cell = &mut self.cells[idx - 1];
            if adj_cell.neighbours() > 0 && !adj_cell.mine() && !adj_cell.covered() {
                if adj_cell.covered() {
                    *adj_cell ^= Cell::COVERED;
                }
            } else {
                self.uncover_blank_neighbours(idx - 1);
            }
        }
        if c < C - 1 {
            let adj_cell = &mut self.cells[idx + 1];
            if adj_cell.neighbours() > 0 && !adj_cell.mine() && !adj_cell.covered() {
                if adj_cell.covered() {
                    *adj_cell ^= Cell::COVERED;
                }
            } else {
                self.uncover_blank_neighbours(idx + 1);
            }
        }
        if r < (N / C) - 1 {
            let adj_cell = &mut self.cells[idx + C];
            if adj_cell.neighbours() > 0 && !adj_cell.mine() && !adj_cell.covered() {
                if adj_cell.covered() {
                    *adj_cell ^= Cell::COVERED;
                }
            } else {
                self.uncover_blank_neighbours(idx + C);
            }
        }
    }
}

impl Cell {
    #[inline(always)]
    pub fn mine(&self) -> bool {
        self.bits() & 1 == 1
    }
    #[inline(always)]
    pub fn covered(&self) -> bool {
        (self.bits() >> 1) & 1 == 1
    }

    #[inline(always)]
    pub fn flagged(&self) -> bool {
        (self.bits() >> 2) & 1 == 1
    }

    #[inline(always)]
    pub fn neighbours(&self) -> u8 {
        // self >> 3 =>  0b000xxxxxx
        self.bits() >> 3
    }

    #[inline(always)]
    pub fn set_neighbours(&mut self, value: u8) {
        // self       =>  0bxxxxxFCM
        //             &
        // 255 >> 5   =>  0b00000111
        //             =  0b00000FCM
        // value << 3 =>  0byyyyy000
        //             |
        //             =  0byyyyyFCM
        self.0 .0 = (self.bits() & (255 >> 5)) | (value << 3);
    }

    #[inline(always)]
    pub fn inc_neighbours(&mut self) {
        let nbors = self.neighbours();
        self.set_neighbours(nbors + 1);
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    macro_rules! test_smol {
        ($cell:ident => [$($n:literal),*]) => {
            $({
                $cell.set_neighbours($n);
                assert_eq!($cell.neighbours(), $n);
            })*
        };
    }

    #[test]
    fn using_the_high_5_bits_in_the_bitflags_actually_works() {
        let mut c = Cell::MINE;
        test_smol! {
            c => [
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31
            ]
        };
        for i in 0..2_u8.pow(5) {
            c.inc_neighbours();
            assert_eq!(c.neighbours(), i)
        }
    }
}
