#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectData {
    width: u16,
    height: u16,
    x: f64,
    y: f64,
    visible: bool,
    id: u16
}