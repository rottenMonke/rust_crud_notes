// Label
// text - string
use serde::Serialize;

#[derive(Serialize)]
pub struct Label {
    pub id: String,
    pub text: String,
    pub color: String,
}
