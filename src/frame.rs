use std::collections::HashMap;

pub struct Frame<'a> {
    pub vars: HashMap<&'a str, f32>
}

impl<'a> Frame<'a> {
    pub fn get_var(&self, key: &str) -> f32{
        match self.vars.get(&key) {
            Some(&ans) => ans,
            _ => 0.0
        }
    }

    pub fn set_var(&mut self, key: &'a str, value: f32) {
        self.vars.insert( key, value);
    }
}
