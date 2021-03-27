#[derive(Default)]
pub struct Score {
    pub score : i32,
    pub block_count : i32,
    pub is_game : bool,
}

impl Score {
    pub fn new() -> Score {
        Score {
            score : 0,
            block_count : 0,
            is_game : false,
        }
    }
    
    pub fn add_score(&mut self, score: i32) {
        self.score += score;
    }

    pub fn add_block_count(&mut self, count: i32) {
        self.block_count += count;
    }

    pub fn subtract_block_count(&mut self, count: i32) {
        self.block_count -= count;
    }
    
    pub fn set_is_game(&mut self, is_game: bool) {
        self.is_game = is_game;
    }
}
