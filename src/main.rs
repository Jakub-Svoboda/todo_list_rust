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
}

fn main() {
    let my_ticket = Ticket::new("My first ticket".to_string());
    let my_ticket_2 = Ticket::new("My second ticket".to_string());

    let mut app = Application::new();
    app.add_ticket(my_ticket);
    app.add_ticket(my_ticket_2);
    app.display();
}
