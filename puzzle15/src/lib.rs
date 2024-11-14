mod circuits;

use std::collections::{HashMap, HashSet, VecDeque};
use std::default;
use std::hash::{Hash, Hasher};
use std::ptr::eq;
use std::thread::current;
use std::{fmt::format, usize};

/// Holds information about which tile is in which position.
/// Should be fairly compact and easy to copy.
#[derive(Debug, Clone, Hash)] // Add Hash here
pub struct GameState {
    board: [[Option<u8>; 4]; 4],
}

/// Creates the default position of tiles, starting with 1 in the top left corner.
impl Default for GameState {
    fn default() -> Self {
        GameState {
            board: [
                [Some(1), Some(5), Some(9), Some(13)],
                [Some(2), Some(6), Some(10), Some(14)],
                [Some(3), Some(7), Some(11), Some(15)],
                [Some(4), Some(8), Some(12), None],
            ],
        }
    }
}

/// Generates a human-readable representation of the game state.
impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display_state = String::new();
        for i in 0..4 {
            display_state.push_str("|");
            for j in 0..4 {
                match self.board[j][i] {
                    Some(val) => display_state.push_str(&format!(" {:>2} |", val)),
                    None => display_state.push_str(&format!("    |")),
                }
            }
            display_state.push('\n');
        }
        write!(f, "{}", display_state)
    }
}

/// Checks whether two game states are the same,.
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            //can just compare the arrays directly
            for j in 0..4 {
                if self.board[i][j] != other.board[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

/// Feel free to ignore this. (but do not remove)
impl Eq for GameState {}

impl GameState {
    /// Updates a position with a new tile.
    pub fn set(&mut self, x: u8, y: u8, tile: Option<u8>) {
        let (x, y) = (x as usize, y as usize);
        self.board[x][y] = tile
    }

    /// Returns the tile at position x,y.
    pub fn get(&self, x: u8, y: u8) -> Option<u8> {
        let (x, y) = (x as usize, y as usize);
        self.board[x][y]
    }

    /// Returns false if there is a duplicate tile in this game state.
    pub fn all_tiles_unique(&self) -> bool {
        let mut my_set: HashSet<Option<u8>> = HashSet::new();
        for i in 0..4 {
            for j in 0..4 {
                if my_set.contains(&self.board[i][j]) {
                    return false;
                }
                if self.board[i][j].is_some()
                    && (self.board[i][j].unwrap() > 15 || self.board[i][j].unwrap() < 1)
                {
                    return false;
                }
                my_set.insert(self.board[i][j]);
            }
        }
        return true;
    }

    /// Swaps the tile from (x1,y1) with the tile from (x2,y2)
    pub fn swap(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        let (x1, y1) = (x1 as usize, y1 as usize);
        let (x2, y2) = (x2 as usize, y2 as usize);
        let temp_tile = self.board[x1][y1];
        self.board[x1][y1] = self.board[x2][y2];
        self.board[x2][y2] = temp_tile;
    }
    fn find_empty_tile(&self) -> Option<(u8, u8)> {
        for x in 0..4 {
            for y in 0..4 {
                if self.get(x, y).is_none() {
                    return Some((x as u8, y as u8));
                }
            }
        }
        None
    }
    /// Updates the state to reflect the move that was performed. Returns false if the move was
    /// not possible.
    pub fn perform_move(&mut self, m: Move) -> bool {
        let Some((col, row)) = self.find_empty_tile() else {
            return false;
        };
        match m {
            Move::LeftToRight => {
                if col == 0 {
                    return false;
                }
                self.swap(col, row, col - 1, row);
                true
            }
            Move::RightToLeft => {
                if col == 3 {
                    return false;
                }
                self.swap(col, row, col + 1, row);
                true
            }
            Move::TopToBottom => {
                if row == 0 {
                    return false;
                }
                self.swap(col, row, col, row - 1);
                true
            }
            Move::BottomToTop => {
                if row == 3 {
                    return false;
                }
                self.swap(col, row, col, row + 1);
                true
            }
        }
    }

    /// Performs a series of moves. Returns the number of moves that were successful.
    pub fn perform_moves(&mut self, moves: &[Move]) -> usize {
        let mut count = 0;
        for m in moves {
            if self.perform_move(*m) {
                count += 1;
            }
        }
        count
    }

    /// Tries to parse a game state from the provided string.
    /// Returns None if parsing is not possible, or if the parsed game state would contain
    /// duplicate or invalid tiles.
    /// Ignores whitespace.

    pub fn from_str(s: &str) -> Option<Self> {
        let mut board = [[None; 4]; 4];
        let mut seen_tiles = HashSet::new();
        let mut empty_tile_count = 0;

        let rows: Vec<&str> = s.trim().lines().collect();
        if rows.len() != 4 {
            return None;
        }

        for (i, row) in rows.iter().enumerate() {
            let cols: Vec<&str> = row.split('|').collect(); //collect makes a new fieled where it stores changes made on immutable
            if cols.len() != 6 {
                //empty cols at start and end
                return None;
            }

            for (j, tile) in cols[1..5].iter().enumerate() {
                let tile = tile.trim();
                if tile.is_empty() {
                    board[j][i] = None;
                    empty_tile_count += 1;
                    if empty_tile_count > 1 {
                        return None;
                    }
                } else if let Ok(val) = tile.parse::<u8>() {
                    if val < 1 || val > 15 || !seen_tiles.insert(val) {
                        //much better than .contains check i orginially did
                        return None;
                    }
                    board[j][i] = Some(val);
                } else {
                    return None;
                }
            }
        }

        Some(GameState { board })
    }
}

/// Finds the minimal number of moves needed to get from one state to the other.
/// Might run forever if there is no path, so use with caution!
pub fn find_shortest_path(from: GameState, to: GameState) -> Vec<Move> {
    if from == to {
        return vec![];
    }
    const MAX_DEPTH: usize = 10000;
    //  to store each state with the path of moves to reach it as key val pair
    let mut possible_states: HashMap<GameState, Vec<Move>> = HashMap::new();
    let mut queue = VecDeque::new();
    //original state so that it doesn't get store later
    possible_states.insert(from.clone(), vec![]);
    queue.push_back(from);

    while !possible_states.contains_key(&to) {
        if possible_states.len() > MAX_DEPTH {
            return vec![]; // Return an empty path if the depth limit is exceeded
        }
        if let Some(current_state) = queue.pop_front() {
            //popping and pushing order doesn't really matter
            let current_path = possible_states.get(&current_state).unwrap().clone();

            let moves = [
                Move::LeftToRight,
                Move::RightToLeft,
                Move::TopToBottom,
                Move::BottomToTop,
            ];
            for &m in &moves {
                let mut new_state = current_state.clone();
                //if its valid move
                if new_state.perform_move(m) {
                    // If it isn't already seen
                    if !possible_states.contains_key(&new_state) {
                        let mut new_path = current_path.clone();
                        new_path.push(m);
                        possible_states.insert(new_state.clone(), new_path);
                        queue.push_back(new_state.clone());

                        if new_state == to {
                            return possible_states.get(&to).unwrap().clone();
                        }
                    }
                }
            }
        } else {
            break;
        }
    }

    vec![]
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u8)]
pub enum Move {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

#[cfg(test)]
mod tests {
    use core::fmt;

    use super::*;
    #[test]
    fn tet_size() {
        assert_eq!(1, std::mem::size_of::<u8>());
        assert_eq!(2, std::mem::size_of::<Option<u8>>());
        assert_eq!(4 * 4 * 2, std::mem::size_of::<GameState>())
    }
    #[test]
    fn test_default_game_state() {
        let state = GameState::default();
        assert_eq!(state.get(0, 0), Some(1));
        assert_eq!(state.get(1, 0), Some(2));
        assert_eq!(state.get(2, 0), Some(3));
        assert_eq!(state.get(3, 0), Some(4));
        assert_eq!(state.get(0, 1), Some(5));
        assert_eq!(state.get(1, 1), Some(6));
        assert_eq!(state.get(2, 1), Some(7));
        assert_eq!(state.get(3, 1), Some(8));
        assert_eq!(state.get(0, 2), Some(9));
        assert_eq!(state.get(1, 2), Some(10));
        assert_eq!(state.get(2, 2), Some(11));
        assert_eq!(state.get(3, 2), Some(12));
        assert_eq!(state.get(0, 3), Some(13));
        assert_eq!(state.get(1, 3), Some(14));
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
    }

    #[test]
    fn test_set_game_state() {
        let mut state = GameState::default();
        state.set(0, 2, Some(3));
        assert_eq!(state.get(0, 2), Some(3));
        state.set(0, 1, Some(4));
        assert_eq!(state.get(0, 1), Some(4));
        // Value of 5 set to tile (0,3) and assert_ne checks if that value is not given to field (0,2)
        state.set(0, 3, Some(5));
        assert_ne!(state.get(0, 2), Some(5));
    }

    const DEFAULT_STATE_STR: &'static str = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
| 13 | 14 | 15 |    |
";

    #[test]
    fn test_display_game_state() {
        let state: GameState = GameState::default();
        assert_eq!(DEFAULT_STATE_STR, format!("{state}"));
        // TODO: add more tests
    }

    #[test]
    fn test_validate_game_state() {
        let mut state = GameState::default();
        assert!(state.all_tiles_unique());
        state.set(3, 0, Some(1));
        assert!(!state.all_tiles_unique());
        state.set(0, 0, Some(4));
        assert!(state.all_tiles_unique());
        // Test that checks that when giving another tile value of 4, not all tiles are unique
        state.set(1, 1, Some(4));
        assert!(!state.all_tiles_unique());
    }

    #[test]
    fn test_swap() {
        let mut state = GameState::default();
        assert_eq!(state.get(2, 3), Some(15));
        assert_eq!(state.get(3, 3), None);
        state.swap(2, 3, 3, 3);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(2, 3), None);
        assert_eq!(state.get(3, 3), Some(15));
        state.swap(0, 0, 2, 2);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(0, 0), Some(11));
        state.swap(3, 3, 1, 1);
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(1, 1), Some(15));
        assert_eq!(state.get(3, 3), Some(6));
    }

    #[test]
    fn test_perform_move() {
        let mut state = GameState::default();
        assert!(!state.perform_move(Move::RightToLeft));
        assert!(!state.perform_move(Move::BottomToTop));
        assert!(state.perform_move(Move::TopToBottom));
        assert!(state.all_tiles_unique());
        assert_eq!(state.get(3, 3), Some(12));
        assert_eq!(state.get(3, 2), None);
        assert!(state.perform_move(Move::LeftToRight));
        assert_eq!(state.get(3, 2), Some(11));
        assert_eq!(state.get(2, 2), None);
        assert!(state.perform_move(Move::RightToLeft));
        assert_eq!(state.get(3, 2), None);
        assert_eq!(state.get(2, 2), Some(11));
        assert!(state.perform_move(Move::TopToBottom));
        assert_eq!(state.get(3, 2), Some(8));
        assert_eq!(state.get(3, 1), None);
    }

    #[test]
    fn test_game_state_equality() {
        let mut state = GameState::default();
        assert!(!state.perform_move(Move::BottomToTop));
        assert_eq!(state, GameState::default());
        assert!(state.perform_move(Move::TopToBottom));
        let mut state_2 = GameState::default();
        state_2.set(3, 3, Some(12));
        state_2.set(3, 2, None);
        assert_eq!(state, state_2);
        assert!(state.perform_move(Move::LeftToRight));
        state_2.set(2, 2, None);
        state_2.set(3, 2, Some(11));
        assert_eq!(state, state_2);
    }

    #[test]
    fn test_perform_moves() {
        let mut state = GameState::default();
        assert_eq!(
            state.perform_moves(&[Move::RightToLeft, Move::BottomToTop, Move::TopToBottom]),
            1
        );

        let mut state = GameState::default();
        assert_eq!(
            state.perform_moves(&[Move::TopToBottom, Move::TopToBottom, Move::TopToBottom]),
            3
        );

        let expected = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        assert_eq!(expected, format!("{state}"));

        let mut state = GameState::default();
        assert_eq!(
            state.perform_moves(&[
                Move::BottomToTop,
                Move::RightToLeft,
                Move::BottomToTop,
                Move::RightToLeft,
                Move::LeftToRight,
                Move::LeftToRight,
                Move::LeftToRight
            ]),
            3
        );
        let expected = "\
|  1 |  2 |  3 |  4 |
|  5 |  6 |  7 |  8 |
|  9 | 10 | 11 | 12 |
|    | 13 | 14 | 15 |
";
        assert_eq!(expected, format!("{state}"));
    }

    #[test]
    fn test_parse_state() {
        assert_eq!(
            GameState::from_str(DEFAULT_STATE_STR).unwrap(),
            GameState::default()
        );

        let wrong0 = "\
|  1 | 22 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong1 = "\
|  1 |  2 ,  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong2 = "\
|  1 |  2 |  3 |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong3 = "\
|  1 |  2 |  3 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
| 13 | 14 | 15 | 12 |
";
        let wrong4 = "\
|  1 |  2 |  3 |    | 1 |
|  5 |  6 |  7 |  4 | 1 |
|  9 | 10 | 11 |  8 | 1 |
| 13 | 14 | 15 | 12 | 1 |
";
        let wrong5 = "\
|  1 |  2 |  3 |    |
|  5 |  2 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";
        let wrong6 = "\
|    |    |    |    |
|    |    |    |    |
|    |    |    |    |
|    |    |    |    |
";

        let wrong7 = "\
|  1 |  2 | 16 |    |
|  5 |  6 |  7 |  4 |
|  9 | 10 | 11 |  8 |
| 13 | 14 | 15 | 12 |
";

        assert!(GameState::from_str(wrong0).is_none());
        assert!(GameState::from_str(wrong1).is_none());
        assert!(GameState::from_str(wrong2).is_none());
        assert!(GameState::from_str(wrong3).is_none());
        assert!(GameState::from_str(wrong4).is_none());
        assert!(GameState::from_str(wrong5).is_none());
        assert!(GameState::from_str(wrong6).is_none()); //should still be wrong since none should be unique too...
        assert!(GameState::from_str(wrong7).is_none());
    }

    #[test]
    fn test_find_shortest_path() {
        let expected_moves = [Move::TopToBottom, Move::TopToBottom, Move::TopToBottom];
        let mut state = GameState::default();
        assert_eq!(state.perform_moves(&expected_moves), 3);

        let actual_moves = find_shortest_path(GameState::default(), state);
        assert_eq!(actual_moves.len(), 3);
        assert_eq!(actual_moves, expected_moves);

        //this added test checks that shortest path will recognize that there is no need to do
        // last 4 moves as they cancel each other out.

        let mut state2 = GameState::default();
        let expected_moves2 = [
            Move::TopToBottom,
            Move::TopToBottom,
            Move::LeftToRight,
            Move::LeftToRight,
            Move::BottomToTop,
            Move::TopToBottom,
            Move::BottomToTop,
            Move::TopToBottom,
        ];
        assert_eq!(state2.perform_moves(&expected_moves2), 8);
        let actual_moves = find_shortest_path(GameState::default(), state2);
        assert_eq!(actual_moves.len(), 4);

        // this test is for finding path to the default state
        let state = GameState::default();
        let moves = find_shortest_path(state.clone(), state.clone());
        assert_eq!(moves.len(), 0);

        let mut state_invalid = GameState::default();
        state_invalid.set(0, 0, Some(16)); // Set an invalid tile so its impossible
        let moves = find_shortest_path(GameState::default(), state_invalid);
        assert!(moves.is_empty());

        let mut state3 = GameState::default();
        let expected_moves3 = [
            Move::TopToBottom,
            Move::LeftToRight,
            Move::TopToBottom,
            Move::RightToLeft,
        ];
        assert_eq!(state3.perform_moves(&expected_moves3), 4);
        let actual_moves3 = find_shortest_path(GameState::default(), state3);
        assert_eq!(actual_moves3.len(), 4);
        assert_eq!(actual_moves3, expected_moves3);
    }
}
