#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::sync::Mutex;
use rocket::State;
use rocket::http::Status;




#[derive(Serialize, Deserialize, Clone)]
struct Ticket {
    id: u64,
    text: String
}

impl Ticket {
    fn new(id: u64, text: String) -> Ticket {
        Ticket { id, text }
    }

    fn display(& self){
        println!("{}:{}", self.id, self.text);
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

    fn find_new_ticket_id(&self) -> u64 {
        let mut max_id = 0;
        for ticket in self.tickets.lock().unwrap().iter() {
            if ticket.id > max_id {
                max_id = ticket.id;
            }
        }
        max_id + 1
    }
}


#[get("/")]
fn index(app: &State<Application>) -> Json<Vec<Ticket>> {
    let tickets = app.tickets.lock().unwrap().clone();
    Json(tickets)
}

#[get("/ticket/<id>")]
fn ticket_detail(id: u64, app: &State<Application>) -> Result<Json<Ticket>, Status> {
    let tickets = app.tickets.lock().unwrap();
    for ticket in tickets.iter() {
        eprintln!("{}:{}", ticket.id, id);
        if ticket.id == id {
            return Ok(Json(ticket.clone()));
        }
    }
    Err(Status::NotFound)
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
        .mount("/", routes![index, ticket_detail])
}
