#![feature(
    proc_macro_hygiene,
    decl_macro,
    register_attr, // Rust replaced the previous feature with this
    rustc_private,
    type_ascription
)]
#[macro_use]
extern crate rocket;

extern crate ws;

use std::thread;

mod route;
use crate::route::{get, static_files};

mod chat;
use crate::chat::ws_rs;

fn rocket() -> rocket::Rocket {
    let rocket_routes = routes![
        static_files::file,
        //
        get::index,
        get::chat,
    ];

    rocket::ignite()
        .mount("/", rocket_routes)
}            // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
// and no need for reconnect later

// https://ws-rs.org/api_docs/ws/struct.Request.html
println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
println!("Client found is {:?}", req.client_addr().unwrap());
let resp = Response::from_request(req);
// println!("{:?} \n", &resp);
resp
}

_ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
}
}

fn on_open(&mut self, handshake: Handshake) -> Result<()> {
// We have a new connection, so we increment the connection counter
self.count.set(self.count.get() + 1);
let number_of_connection = self.count.get();

if number_of_connection > 5 {
// panic!("There are more user connection than expected.");
}

// The most important part and used to assign id for clients
// println!("{}", &handshake.local_addr.unwrap());
let open_message = format!("{} entered and the number of live connections is {}", &handshake.peer_addr.unwrap(), &number_of_connection);

println!("{}", &open_message);
self.out.broadcast(open_message);

Ok(())
}

// Handle messag            // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
                // and no need for reconnect later

                // https://ws-rs.org/api_docs/ws/struct.Request.html
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());
                let resp = Response::from_request(req);
                // println!("{:?} \n", &resp);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        if number_of_connection > 5 {
            // panic!("There are more user connection than expected.");
        }
        
        // The most important part and used to assign id for clients
        // println!("{}", &handshake.local_addr.unwrap());
        let open_message = format!("{} entered and the number of live connections is {}", &handshake.peer_addr.unwrap(), &number_of_connection);

        println!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }

    // Handle messag            // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
                // and no need for reconnect later

                // https://ws-rs.org/api_docs/ws/struct.Request.html
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());
                let resp = Response::from_request(req);
                // println!("{:?} \n", &resp);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        if number_of_connection > 5 {
            // panic!("There are more user connection than expected.");
        }
        
        // The most important part and used to assign id for clients
        // println!("{}", &handshake.local_addr.unwrap());
        let open_message = format!("{} entered and the number of live connections is {}", &handshake.peer_addr.unwrap(), &number_of_connection);

        println!("{}", &open_message);
        self.out.broadcast(open_message);

        Ok(())
    }
}

// Handle messag            // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
            // and no need for reconnect later

            // https://ws-rs.org/api_docs/ws/struct.Request.html
            println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
            println!("Client found is {:?}", req.client_addr().unwrap());
            let resp = Response::from_request(req);
            // println!("{:?} \n", &resp);
            resp
        }

        _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
    }
}

fn on_open(&mut self, handshake: Handshake) -> Result<()> {
    // We have a new connection, so we increment the connection counter
    self.count.set(self.count.get() + 1);
    let number_of_connection = self.count.get();

    if number_of_connection > 5 {
        // panic!("There are more user connection than expected.");
    } }

    // Handle messag            // https://blog.stanko.io/do-you-really-need-websockets-343aed40aa9b
                // and no need for reconnect later

                // https://ws-rs.org/api_docs/ws/struct.Request.html
                println!("Browser Request from {:?}", req.origin().unwrap().unwrap());
                println!("Client found is {:?}", req.client_addr().unwrap());
                let resp = Response::from_request(req);
                // println!("{:?} \n", &resp);
                resp
            }

            _ => Ok(Response::new(404, "Not Found", b"404 - Not Found".to_vec())),
        }
    }

    fn on_open(&mut self, handshake: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        self.count.set(self.count.get() + 1);
        let number_of_connection = self.count.get();

        if number_of_connection > 5 {
            // panic!("There are more user connection than expected.");
        }
    // Handle messag

// https://rust-lang.github.io/async-book/getting_started/why_async.html
fn main() {
    // $rustapi and search thread
    thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        // .stack_size(83886 * 1024) // 80mib in killobytes
        .spawn(|| {
            ws_rs::websocket();
        })
        .unwrap();

    rocket().launch();
}

