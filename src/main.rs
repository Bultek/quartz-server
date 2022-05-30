#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};
pub static mut MESSAGES: Vec<Message> = Vec::new();

pub struct Message {
    message: String,
    contact: String,
    sender: String,
}

impl Message {
    pub fn new(message: String, contact: String, sender: String) -> Message {
        Message {
            message,
            contact,
            sender,
        }
    }
    pub fn clone(msg: Message) -> Message {
        Message {
            message: msg.message,
            contact: msg.contact,
            sender: msg.sender,
        }
    }
    pub fn iter() -> impl Iterator<Item = &'static Message> {
        unsafe { MESSAGES.iter() }
    }
    #[allow(non_snake_case)]
    pub fn Serialize(msg: Message) -> Json<Message> {
        Json(msg)
    }
}

// Accept JSON posts
#[post("/incoming", format = "json", data = "<target>")]
fn incoming(target: &str) {
    // Decode the JSON into a hashmap
    let data: serde_json::Value = serde_json::from_str(target).unwrap();
    // Get the data
    let message = data["message"].as_str().unwrap();
    let sender = data["sender"].as_str().unwrap();
    let contact = data["contact"].as_str().unwrap();
    // Return the message
    let _m = Message {
        message: String::from(message),
        contact: String::from(contact),
        sender: String::from(sender),
    };
    unsafe {
        MESSAGES.push(_m);
        for _m in &MESSAGES {
            println!("{}", _m.contact);
            println!("{}", _m.sender);
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct T {
    messages: Vec<String>,
    senders: Vec<String>,
    contacts: Vec<String>,
}

pub fn cpymsgvec(a: &Vec<Message>) -> Vec<Message> {
    let mut v: Vec<Message> = Vec::new();
    for mess in a {
        let msg = Message::new(
            mess.message.clone(),
            mess.contact.clone(),
            mess.sender.clone(),
        );
        v.push(msg);
    }
    v
}

#[get("/messages")]
async fn give_messages() -> Json<T> {
    let mut _messages: Vec<Message> = Vec::new();
    unsafe {
        // Copy messages
        _messages = cpymsgvec(&MESSAGES);
        for _m in &_messages {
            println!("{}", _m.contact);
        }
    }
    //let mut _ms: Vec<Message> = Vec::new();
    let mut msgsenders: Vec<String> = Vec::new();
    let mut msmsgs: Vec<String> = Vec::new();
    let mut contacts: Vec<String> = Vec::new();
    for msg in _messages {
        let c = msg.contact.clone();
        let q = msg.sender.clone();
        let w = msg.message.clone();
        msgsenders.push(msg.sender);
        msmsgs.push(msg.message);
        contacts.push(msg.contact);
        println!("{}", w);
        println!("{}", q);
        println!("{}", c);
    }
    let hsh = T {
        messages: msmsgs,
        senders: msgsenders,
        contacts,
    };
    // Return the JSON
    Json(hsh)
}

#[launch]
async fn rocket() -> _ {
    // Start the server
    rocket::build().mount("/", routes![incoming, give_messages])
}
