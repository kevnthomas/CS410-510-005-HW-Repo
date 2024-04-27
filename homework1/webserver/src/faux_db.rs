use crate::question::{Question, QuestionId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

impl Store {
    /*
    pub fn init(self) -> Self {
        let question = Question::new(
            QuestionId::from_str("1").expect("Id not set"),
            "How?".to_string(),
            "Please help!".to_string(),
            Some(vec!["general".to_string()]),
        );
        self.add_question(question)
    }
    */

    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    /*
    pub fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }
    */
    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!(r"questions.json");
        serde_json::from_str(file).expect("cant read questions.json")
    }
}
