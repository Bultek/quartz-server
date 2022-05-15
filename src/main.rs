#[macro_use]
extern crate rocket;
use rocket::serde::{Serialize, json::Json};
pub static mut MESSAGES: Vec<Message> = Vec::new();

#[allow(dead_code)]
pub struct Message {
    message: String,
    contact: String,
    sender: String,
}

pub trait Iterator {
    type Message;
}

pub trait Clone {
    type Message;
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


pub fn cpymessagevec(_messages: &Vec<Message>) -> Vec<Message> {
    
    let mut _ms: Vec<Message> = Vec::new();
    for _ms in _messages{
        let _m = Message {
            message: _ms.message.clone().to_string(),
            contact: _ms.contact.clone().to_string(),
            sender:  _ms.sender.clone().to_string(),
        };
        //println!("{}",_ms.message);
    }
    return _ms;
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct T {
    messages: Vec<String>,
    senders: Vec<String>,
}

#[get("/messages/<contact>")]
async fn give_messages(contact: String) -> Json<T> {
    
    let mut _messages: Vec<Message> = Vec::new();
    unsafe {
        // Copy messages
        _messages = cpymessagevec(&MESSAGES);
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
    rocket::build()
        .mount("/", routes![incoming,give_messages])
}
