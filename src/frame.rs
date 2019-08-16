use std::collections::HashMap;
use crate::stackobj::StackObj;

pub struct Frame<'a> {
    pub vars: HashMap<&'a str, StackObj>,
    pub return_address: i32
}

impl<'a> Frame<'a> {
    pub fn get_var(&self, key: &str) -> StackObj {
        match self.vars.get(&key) {
            Some(&ans) => ans,
            _ => StackObj::Num(0.0),
        }
    }

    pub fn set_var(&mut self, key: &'a str, value: StackObj) {
        self.vars.insert( key, value);
    }
}
