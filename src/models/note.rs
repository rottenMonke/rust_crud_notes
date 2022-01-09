// Note
// text - Just plain text
// label - Label[]
// title - text
// comment - text
// creation_date - UTC
// updated_date - UTC
// remind_me_about_it - UTC[]
// author - id (string)
// group - note group id
use super::label::Label;
use serde::Serialize;

#[derive(Serialize)]
pub struct Note {
    pub id: String,
    pub text: String,
    pub labels: Vec<Label>,
    pub title: String,
    pub comment: String,
    pub creation_date: String,
    pub remind_me_about_it: Vec<String>,
    pub author: String,
    pub group: String,
}
