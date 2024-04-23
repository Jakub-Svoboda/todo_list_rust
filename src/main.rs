use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;


#[derive(Serialize, Deserialize)]
struct Ticket {
    text: String
}

impl Ticket {
    fn new(text: String) -> Ticket {
        Ticket { text }
    }

    fn display(& self){
        println!("{}", self.text);
    }

    fn set_text(&mut self, text: String){
        self.text = text;
    }

    fn get_text(&self) -> &String{
        return &self.text;
    }
}

#[derive(Serialize, Deserialize)]
struct Application {
    tickets: Vec<Ticket>,
}

impl Application {
    fn new() -> Application {
        Application {
            tickets: Vec::new()
        }
    }

    fn add_ticket(&mut self, ticket: Ticket){
        self.tickets.push(ticket);
    }

    fn display(&self){
        for ticket in &self.tickets {
            ticket.display();
        }
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.tickets)?;

        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;

        Ok(())
    }

    fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let tickets: Vec<Ticket> = serde_json::from_str(&contents)?;

        Ok(Self { tickets })
    }
}

fn main() {
    let my_ticket = Ticket::new("My first ticket".to_string());
    let my_ticket_2 = Ticket::new("My second ticket".to_string());

    let mut app = Application::new();
    app.add_ticket(my_ticket);
    app.add_ticket(my_ticket_2);
    app.display();

    if let Err(e) = app.save_to_file("tickets.json") {
        eprintln!("Failed to save tickets: {}", e);
    }

    let loaded_app = Application::load_from_file("tickets.json").unwrap();
    loaded_app.display();
    let my_ticket_3 = Ticket::new("My third ticket".to_string());

    if let Err(e) = app.save_to_file("tickets.json") {
        eprintln!("Failed to save tickets: {}", e);
    }

}
