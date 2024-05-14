use serde::{Serialize, Deserialize};

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


#[derive(Serialize, Deserialize)]
pub struct TicketForm {
    pub text: String,
}
