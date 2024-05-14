use crate::objects::ticket::Ticket;

use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::sync::Mutex;



#[derive(Serialize, Deserialize)]
pub struct Application {
    pub tickets: Mutex<Vec<Ticket>>,
}


impl Application {
    fn new() -> Application {
        Application {
            tickets: Mutex::new(Vec::new()),
        }
    }

    pub fn add_ticket(&self, ticket: Ticket) {
        self.tickets.lock().unwrap().push(ticket);
    }

    pub fn edit_ticket(&self, id: u64, text: String) -> bool {
        let mut tickets = self.tickets.lock().unwrap();
        for ticket in tickets.iter_mut() {
            if ticket.id == id {
                ticket.text = text.clone();
                return true;
            }
        }
        return false;
    }

    pub fn display(&self){
        for ticket in self.tickets.lock().unwrap().iter() {
            ticket.display();
        }
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.tickets)?;

        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let tickets: Vec<Ticket> = serde_json::from_str(&contents)?;

        Ok(Self { tickets: Mutex::new(tickets) })
    }

    pub fn find_new_ticket_id(&self) -> u64 {
        let mut max_id = 0;
        for ticket in self.tickets.lock().unwrap().iter() {
            if ticket.id > max_id {
                max_id = ticket.id;
            }
        }
        max_id + 1
    }
}
