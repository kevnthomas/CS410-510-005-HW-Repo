// Kevin Thomas CS410P:Rust Web Dev
//main.rs
//implementation of the struct question
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind};
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct QuestionId(pub String);

/*
impl Question {
    pub fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}
*/

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided.")),
        }
    }
}
/*
impl std::fmt::Display for Question{
    fn fmt(
        &self, f: &mut std::fmt::Formatter
    ) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}, title: {}, content : {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(
        &self, f: &mut std::fmt::Formatter
    ) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "id: {}",
            self.0
        )
    }
}

impl std::fmt::Debug for Question {
    fn fmt(
        &self, f: &mut std::fmt::Formatter<'_>
    ) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{:?}",
            self.tags
        )
    }
}
*/

/*
fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    //print with fns i wrote
    //println!("{}", question);
    //print with derive fns
    println!("{:?}", question);
}
*/
