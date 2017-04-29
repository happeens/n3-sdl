use sprite::SpriteManager;
use state::{State, StateData};

pub struct StateManager {
    states: Vec<State>,
    current: usize
}

impl StateManager {
    pub fn new(data: &Vec<StateData>, sprites: &SpriteManager) -> StateManager {
        let mut states = Vec::new();

        for state_data in data.iter() {
            if let Some(index) = sprites.find_index(&state_data.frame) {
                states.push(State::new(&state_data.name, index));
            }
        }

        StateManager {
            states: states,
            current: 0
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn set(&mut self, name: &str) {
        for state in self.states.iter() {
            if state.name == name {
                self.current = state.index;
                return;
            }
        }

        println!("state not found: {}", name);
    }
}