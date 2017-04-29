mod state_manager;
pub type StateManager = state_manager::StateManager;

#[derive(Serialize, Deserialize, Debug)]
pub struct StateData {
    name: String,
    frame: String
}

pub struct State {
    pub name: String,
    pub index: usize
}

impl State {
    pub fn new(name: &str, index: usize) -> State {
        State {
            name: String::from(name),
            index: index
        }
    }
}