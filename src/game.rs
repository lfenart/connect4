use mcts::*;

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    player: Player,
    state: State,
    moves: u8,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player: Player::new(),
            state: State::Unfinished,
            moves: 0,
        }
    }

    pub fn can_play(&self, action: u8) -> bool {
        self.board.can_play(action)
    }

    pub fn state(&self) -> State {
        self.state
    }
}

impl MctsGame for Game {
    type Player = Player;
    type Action = u8;

    fn legal_actions(&self) -> Vec<Self::Action> {
        let mut actions = Vec::new();
        for i in 0..Board::WIDTH {
            if self.board.can_play(i) {
                actions.push(i);
            }
        }
        actions
    }

    fn play(&mut self, action: Self::Action) {
        let result = self.board.play(action);
        self.moves += 1;
        self.player = self.player().other();
        if result && self.state == State::Unfinished {
            self.state = State::Win(self.player());
        } else if self.moves >= Board::WIDTH * Board::HEIGHT {
            self.state = State::Draw;
        }
    }

    fn player(&self) -> Self::Player {
        self.player
    }

    fn state(&self, player: Self::Player) -> MctsState {
        match self.state {
            State::Unfinished => MctsState::Unfinished,
            State::Draw => MctsState::Draw,
            State::Win(p) if p == player => MctsState::Win,
            _ => MctsState::Lose,
        }
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (c1, c2) = if self.player() == Player(1) {
            ('X', '0')
        } else {
            ('0', 'X')
        };
        let mut result = String::new();
        for i in 0..Board::WIDTH {
            result.push(' ');
            result.push((i + b'1') as char);
        }
        for i in (0..Board::HEIGHT).rev() {
            result.push('\n');
            result.push('|');
            for j in 0..Board::WIDTH {
                if self.board.mask & ((1 << i) << (Board::MASK_HEIGHT * j)) != 0 {
                    if self.board.player_bitboard & ((1 << i) << (Board::MASK_HEIGHT * j)) != 0 {
                        result.push(c1);
                    } else {
                        result.push(c2);
                    }
                } else {
                    result.push(' ');
                }
                result.push('|');
            }
        }
        result.push('\n');
        for _ in 0..Board::WIDTH {
            result.push('-');
            result.push('-');
        }
        result.push('-');
        result.push(' ');
        result.push(c1);
        write!(f, "{}", result)
    }
}

#[derive(Debug, Clone)]
struct Board {
    mask: u64,
    player_bitboard: u64,
}

impl Board {
    pub const HEIGHT: u8 = 6;
    pub const WIDTH: u8 = 7;
    const MASK_HEIGHT: u8 = Self::HEIGHT + 1;

    fn new() -> Self {
        Self {
            mask: 0,
            player_bitboard: 0,
        }
    }

    fn can_play(&self, column: u8) -> bool {
        self.mask & Self::top_mask(column) == 0
    }

    fn play(&mut self, column: u8) -> bool {
        self.player_bitboard ^= self.mask;
        self.mask |= self.mask + Self::bottom_mask(column);
        self.alignment()
    }

    fn alignment(&self) -> bool {
        #[inline]
        const fn check(pos: u64, shift: u8) -> bool {
            let m = pos & (pos >> shift);
            m & (m >> (2 * shift)) != 0
        }
        let pos = self.player_bitboard ^ self.mask;
        check(pos, 1)
            || check(pos, Self::MASK_HEIGHT)
            || check(pos, Self::MASK_HEIGHT + 1)
            || check(pos, Self::MASK_HEIGHT - 1)
    }

    fn bottom_mask(column: u8) -> u64 {
        1 << (column * Self::MASK_HEIGHT)
    }

    fn top_mask(column: u8) -> u64 {
        Self::bottom_mask(column) << (Self::MASK_HEIGHT - 2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Player(pub u8);

impl Player {
    fn new() -> Self {
        Self(1)
    }

    fn other(self) -> Self {
        Self(3 - self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum State {
    Win(Player),
    Draw,
    Unfinished,
}
