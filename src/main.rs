#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::sync::Mutex;
use rocket::State;




#[derive(Serialize, Deserialize, Clone)]
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
    tickets: Mutex<Vec<Ticket>>,
}

impl Application {
    fn new() -> Application {
        Application {
            tickets: Mutex::new(Vec::new()),
        }
    }

    fn add_ticket(&mut self, ticket: Ticket){
        self.tickets.lock().unwrap().push(ticket);
    }

    fn display(&self){
        for ticket in self.tickets.lock().unwrap().iter() {
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

        Ok(Self { tickets: Mutex::new(tickets) })
    }
}


#[get("/")]
fn index(app: &State<Application>) -> Json<Vec<Ticket>> {
    let tickets = app.tickets.lock().unwrap().clone();
    Json(tickets)
}

#[launch]
fn rocket() -> _ {
    let app = match Application::load_from_file("tickets.json") {
        Ok(app) => app,
        Err(_) => Application {
            tickets: Mutex::new(Vec::new()),
        },
    };

    rocket::build()
        .manage(app)
        .mount("/", routes![index])
}
