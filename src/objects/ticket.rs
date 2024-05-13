use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;

#[derive(Serialize, Deserialize, Clone)]
pub struct Ticket {
    pub id: u64,
    pub text: String
}

impl Ticket {
    pub fn new(id: u64, text: String) -> Ticket {
        Ticket { id, text }
    }

    pub fn display(& self){
        println!("{}:{}", self.id, self.text);
    }

    pub fn set_text(&mut self, text: String){
        self.text = text;
    }

    pub fn get_text(&self) -> &String{
        return &self.text;
    }
}
