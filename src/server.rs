#[macro_use] extern crate rocket;

pub mod objects;
use crate::objects::ticket::Ticket;
use crate::objects::application::Application;

use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use rocket::State;
use rocket::http::Status;



#[get("/")]
fn index(app: &State<Application>) -> Json<Vec<Ticket>> {
    let tickets = app.tickets.lock().unwrap().clone();
    Json(tickets)
}


#[get("/ticket/<id>")]
fn ticket_detail(id: u64, app: &State<Application>) -> Result<Json<Ticket>, Status> {
    let tickets = app.tickets.lock().unwrap();
    for ticket in tickets.iter() {
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
