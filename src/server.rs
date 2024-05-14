#[macro_use] extern crate rocket;

pub mod objects;
use crate::objects::ticket::Ticket;
use crate::objects::ticket::TicketForm;
use crate::objects::application::Application;

use rocket::serde::json::Json;
use std::sync::Mutex;
use rocket::State;
use rocket::http::Status;

fn get_tickets(app: &State<Application>) -> Json<Vec<Ticket>> {
    let tickets = app.tickets.lock().unwrap().clone();
    Json(tickets)
}


#[get("/")]
fn index(app: &State<Application>) -> Json<Vec<Ticket>> {
    get_tickets(app)
}


#[get("/api/v1/ticket")]
fn ticket_list(app: &State<Application>) -> Json<Vec<Ticket>> {
    println!("FOO"); // TODO remove
    get_tickets(app)
}


#[get("/api/v1/ticket/<id>")]
fn ticket_detail(id: u64, app: &State<Application>) -> Result<Json<Ticket>, Status> {
    let tickets = app.tickets.lock().unwrap();
    for ticket in tickets.iter() {
        if ticket.id == id {
            return Ok(Json(ticket.clone()));
        }
    }
    Err(Status::NotFound)
}


#[post("/api/v1/ticket", data = "<ticket_form>")]
fn create_ticket(ticket_form: Json<TicketForm>, app: &State<Application>) -> Json<Ticket> {
    let new_ticket = Ticket {
        id: app.find_new_ticket_id(),
        text: ticket_form.text.clone(),
    };
    let mut tickets = app.tickets.lock().unwrap();
    tickets.push(new_ticket.clone());
    Json(new_ticket)
}

#[put("/api/v1/ticket/<id>", data = "<ticket_form>")]
fn update_ticket(id: u64, ticket_form: Json<TicketForm>, app: &State<Application>) -> Result<Json<Ticket>, Status> {
    let mut tickets = app.tickets.lock().unwrap();
    for ticket in tickets.iter_mut() {
        if ticket.id == id {
            ticket.text = ticket_form.text.clone();
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
        .mount("/", routes![index, ticket_list, ticket_detail, create_ticket, update_ticket])
}
