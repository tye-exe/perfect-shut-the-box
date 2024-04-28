use fastrand::Rng;

use crate::roll::Roll;

// Binary representation of the board within the u16:
// 0000000 | 000000000
// 0000000 | 987654321

/// Contains a current state of the board & the possible moves that could be made for each possible roll.
#[derive(Debug)]
pub struct Board {
    board: u16,
    rolls: Vec<Roll>,
}

/// Contains each possible roll, which amount each value occurs being the weight of the value to be chosen.
const POSSIBLE_ROLLS_INDEXES: [u8; 36] = [0, 1, 1, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 5, 6, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 9, 9, 10];

impl Board {
    /// Simulates the possible rolls & their valid moves for the given board.
    ///
    /// Binary representation of the board within the u16:
    ///  0000000 | 000000000
    ///  0000000 | 987654321
    pub fn new(board: u16) -> Board {
        let mut roles = Vec::with_capacity(11);
        for role in 2u8..13 {
            roles.push(Roll::new(role, board));
        }

        Board {
            board,
            rolls: roles,
        }
    }

    /// Gets a random roll from the board.
    /// The chance of a roll to be returned directly correlates to the chance it will be rolled.
    pub fn get_rand_roll(&self, rng: &mut Rng) -> &Roll {
        let index = rng.usize(..POSSIBLE_ROLLS_INDEXES.len());
        let roll_index = POSSIBLE_ROLLS_INDEXES.get(index).expect("Will never be empty");

        return self.rolls.get(*roll_index as usize).expect("A board always has 11 roles.");
    }

    /// Sums up the numeric value of the alive pieces for this board.
    pub fn calculate_value(&self) -> u8 {
        let mut total_value = 0;

        for index in 0..9 {
            let piece = self.board >> index;

            if piece & 1 == 1 {
                total_value += index + 1;
            }
        }

        total_value
    }

    /// Returns a copy of the raw u16 that represents this board.
    ///
    /// Binary representation of the board within the u16:
    ///  0000000 | 000000000
    ///  0000000 | 987654321
    pub fn get_raw(&self) -> u16 {
        self.board
    }
}