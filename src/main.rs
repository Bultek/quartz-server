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
        let m = Message {
            message: msg.message.clone(),
            contact: msg.contact.clone(),
            sender: msg.sender.clone(),
        };
        return m;
    }
    pub fn iter() -> impl Iterator<Item = &'static Message> {
        unsafe { MESSAGES.iter() }
    }
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
    return v;
}

#[get("/messages/<contact>")]
async fn give_messages(contact: String) -> Json<T> {
    let mut _messages: Vec<Message> = Vec::new();
    unsafe {
        // Copy messages
        _messages = cpymsgvec(&MESSAGES);
        for _m in &_messages {
            println!("{}", _m.contact);
        }
    }
    let mut _ms: Vec<Message> = Vec::new();
    for m in _messages {
        if m.contact == contact {
            _ms.push(m);
        }
    }
    let mut msgsenders: Vec<String> = Vec::new();
    let mut msmsgs: Vec<String> = Vec::new();
    for msg in _ms {
        let q = msg.sender.clone();
        let w = msg.message.clone();
        msgsenders.push(msg.sender);
        msmsgs.push(msg.message);
        println!("{}", w);
        println!("{}", q);
    }
    let hsh = T {
        messages: msmsgs,
        senders: msgsenders,
    };
    // Return the JSON
    Json(hsh)
}

#[launch]
async fn rocket() -> _ {
    // Start the server
    rocket::build().mount("/", routes![incoming, give_messages])
}
