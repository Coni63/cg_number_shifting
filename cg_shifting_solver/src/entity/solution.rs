use crate::entity::action::Action;

pub struct Solution {
    pub actions: Vec<Action>,
    pub score: i32,
}

impl Solution {
    pub fn new(actions: Vec<Action>, score: i32) -> Solution {
        Solution { actions, score }
    }
}

impl Clone for Solution {
    fn clone(&self) -> Solution {
        Solution {
            actions: self.actions.clone(),
            score: self.score,
        }
    }
}
