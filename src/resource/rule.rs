use amethyst::prelude::World;

#[derive(Default)]
pub struct Rule {
    pub is_game : bool,
}

impl Rule {
    pub fn new() -> Rule {
        Rule {
            is_game : false,
        }
    }
    
    pub fn set_is_game(&mut self, is_game: bool) {
        self.is_game = is_game;
    }
}

pub fn create_rule( world: &mut World) {
    let rule = Rule::new();
    world.insert(rule);
}
