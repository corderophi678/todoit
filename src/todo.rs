#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub task: String,
    pub completed: bool,
}
