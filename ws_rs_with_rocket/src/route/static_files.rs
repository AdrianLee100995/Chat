use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<file..>")]
pub fn file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
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