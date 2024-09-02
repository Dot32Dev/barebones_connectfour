// Connect Four bitboard implementation as described here:
// https://github.com/denkspuren/BitboardC4/blob/master/BitboardDesign.md
//
// How the bits in a u64 map to the board:
//   6 13 20 27 34 41 48   55 62        Padding row
// ╭─────────────────────╮
// │ 5 12 19 26 33 40 47 │ 54 61        top row
// │ 4 11 18 25 32 39 46 │ 53 60
// │ 3 10 17 24 31 38 45 │ 52 59
// │ 2  9 16 23 30 37 44 │ 51 58
// │ 1  8 15 22 29 36 43 │ 50 57
// │ 0  7 14 21 28 35 42 │ 49 56 63     bottom row
// ╰─────────────────────╯

pub struct Bitboard {
    // Each player has a u64 with each set bit corresponding to a placed piece.
    players: [u64; 2],

    // Stores the bit index of the highest piece in any given column, plus one.
    // This is necessary to know where pieces will land in a move.
    heights: [u8; 7],

    // Counts moves. If the count is even, it's player 1's turn. If it's odd,
    // it's player 2's turn. If the count is 42 the game is a draw.
    pub move_counter: usize,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        // Initialise the heights array to the bottom row of the board: i * 7
        let mut heights: [u8; 7] = [0; 7];
        for i in 1..heights.len() {
            heights[i] = i as u8 * 7;
        }

        Bitboard {
            players: [0; 2],
            heights,
            move_counter: 0,
        }
    }

    pub fn drop_piece(&mut self, column: usize) -> bool {
        // Returns false when unnable to drop piece
        if self.heights[column] == column as u8 * 7 + 6 {
            return false;
        }

        // Shift the bit to the bit described in the heights aray
        let place = 1 << self.heights[column];
        // Add to the height of the column
        self.heights[column] += 1;
        // "self.move_counter & 1" is 1 on and odd move and 0 on an even move
        // |= or's the bitboard with the "place" bit, adding it.
        self.players[self.move_counter & 1] |= place;

        self.move_counter += 1;

        true
    }

    pub fn check_win(&self) -> bool {
        // Because the drop piece method adds to the move counter, switching the
        // player, the most recent piece dropped is actually by the previous
        // player. We calculate this with 1 - "current player", so that if the
        // player is 1, we check for player 0, and if the player is 0 we check 1
        let player = self.players[1 - (self.move_counter & 1)];

        // If you look at the layout of the bitboard, you will notice that the
        // difference between horizontal pieces for example is 7. If we use an
        // and operation between the board, and the board shifted by 7, 14, and
        // 21 bits, we will only not get zero when there is an instance of 4
        // bits horizontally in a row. The only difference between the
        // directions are the magic numbers to shift by. The padding row ensures
        // no "wrapping around" of the diagonal or vertical checks.

        // Check for diagonal \
        if player & (player >> 6) & (player >> 12) & (player >> 18) != 0 {
            return true;
        }

        // Check for diagonal /
        if player & (player >> 8) & (player >> 16) & (player >> 24) != 0 {
            return true;
        }

        // Check for horizontal
        if player & (player >> 7) & (player >> 14) & (player >> 21) != 0 {
            return true;
        }

        // Check for vertical
        if player & (player >> 1) & (player >> 2) & (player >> 3) != 0 {
            return true;
        }

        false
    }
}

// Enables printing the board directly like println!("{}", board)
use std::fmt;
impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print column numbers
        write!(f, "╭ 1  2  3  4  5  6  7 ╮\n")?;
        write!(f, "├─────────────────────┤\n")?;

        // Start at the top row, which is row 7
        for row in (0..6).rev() {
            write!(f, "│")?;

            for col in 0..7 {
                // Calculate the bit index
                let index = row + col * 7;

                // Extract the bit at the index for each player
                // Player 1 is 1, player two is multiplied by 2 to get 2
                let token = ((self.players[0] >> index) & 1)
                    + ((self.players[1] >> index) & 1) * 2;

                write!(f, " {} ", token)?;
            }
            write!(f, "│\n")?;
        }
        write!(f, "╰─────────────────────╯")?;
        Ok(())
    }
}
