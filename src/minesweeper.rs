use std::{collections::HashSet, fmt::{Display, Write}};
use crate::random::*;
use crate::log::*;


pub type Position = (usize, usize);

// impl Display for Position {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "({},{})", self.0, self.1)
//     } 
// }

#[derive(Debug)]
pub enum OpenResult {
    Mine,
    NoMine(u8),
}

#[derive(Debug)]
pub struct Minesweeper {
    width: usize,
    height: usize,
    open_fields: HashSet<Position>,
    mines: HashSet<Position>,
    flags: HashSet<Position>,
    lost: bool
}

impl Display for Minesweeper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.height {
            for c in 0..self.width {
                let position = (r, c);
                if ! self.open_fields.contains(&position) {
                    if self.lost && self.mines.contains(&position) {
                        f.write_str("💣 ")?;
                    } else if self.flags.contains(&position) {
                        f.write_str("🚩 ")?;
                    } else {
                        f.write_str("🟪 ")?;
                    }
                } else if self.mines.contains(&position) {
                    f.write_str("💣 ")?;
                } else {
                    let mine_count = self.neighboring_mines(position);
                    if mine_count == 0 {
                        f.write_str("⬜ ")?;
                    } else {
                        write!(f, " {} ", mine_count)?;
                    }
                }
            }
            f.write_char('\n')?;
        }
        // write!(f, "{:?}", self.mines)?;
        Ok(())
    }
}

impl Minesweeper {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        Minesweeper { 
            width, 
            height, 
            open_fields: HashSet::new(), 
            mines: { 
                let mut mines = HashSet::new();
                while mines.len() <  mine_count {
                    let (r, c) = (random_range(0 , width),
                        random_range(0, height));
                    mines.insert((r, c));
                }
                mines
            }, 
            flags: HashSet::new() ,
            lost: false,
        }
    }

    pub fn iter_neighbors(&self, (r, c): Position) -> impl Iterator<Item = Position> {
        let width = self.width;
        let height = self.height;

        //            0..height
        (r.max(1) - 1..=(r + 1).min(height - 1))
            .flat_map(move |r| 
        //            0..width
            (c.max(1) - 1..=(c + 1).min(width - 1))
                .map(move |c|(r, c)))
        .filter(move |&pos| pos != (r, c))
    }

    pub fn neighboring_mines(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|pos| self.mines.contains(&pos))
            .count() as u8
    }

    pub fn neighboring_flags(&self, position: Position) -> u8 {
        self.iter_neighbors(position)
            .filter(|pos| self.flags.contains(&pos))
            .count() as u8
    }

    pub fn open(&mut self, position: Position) -> Option<OpenResult> {
        // already open
        if self.open_fields.contains(&position) {
            let neighbor_mines = self.neighboring_mines(position);
            let neighbor_flags = self.neighboring_flags(position);
            if neighbor_flags == neighbor_mines && neighbor_mines > 0 {
                for pos in self.iter_neighbors(position) {
                    self._open(pos);
                }
            }
            return None
        } else {
            self._open(position)
        }
    }

    pub fn _open(&mut self, position: Position) -> Option<OpenResult> {
        // already open
        if self.lost || self.flags.contains(&position) || self.open_fields.contains(&position) {
            return None
        }

        // console_log!("open: ({},{}), {self}", position.0, position.1);
        // console_log!("mines: {:?}", self.mines);
        // console_log!("neighbors: {:?}", self.iter_neighbors(position).collect::<Vec<(usize, usize)>>());
        self.open_fields.insert(position);
        let is_mine = self.mines.contains(&position);
        if is_mine {
            self.lost = true;
            Some(OpenResult::Mine)
        } else {
            let neighbor_mines = self.neighboring_mines(position);
            if neighbor_mines == 0 {
                for pos in self.iter_neighbors(position) {
                    let _ = self._open(pos);
                }
            }
            Some(OpenResult::NoMine(neighbor_mines))
        }
    }

    pub fn toggle_flag(&mut self, position: Position) {
        if self.lost || self.open_fields.contains(&position) {
            return;
        }
        if self.flags.contains(&position) {
            self.flags.remove(&position);
        } else {
            self.flags.insert(position);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Minesweeper;

    #[test]

    fn test_new() {
        let mut ms = Minesweeper::new(10, 10, 5);
        ms.open((5, 5));
        ms.toggle_flag((5, 1));
        ms.toggle_flag((7, 3));
        ms.open((2, 2));
        ms.open((1, 1));
        ms.open((4, 4));
        ms.open((5, 5));
        println!("{}", ms);
        println!("{:?}", ms);
    }
}
