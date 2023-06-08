use crate::limit::LimitReq;
use crate::tc::{add_filter, create_qdisc, delete_filter, destroy_qdisc};
use std::io::Read;
use std::net::{TcpListener, TcpStream};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: &str) -> Server {
        Server {
            address: address.to_string(),
        }
    }

    pub fn listen(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.address.clone())?;

        for stream in listener.incoming() {
            self.handle_client(stream?)?;
        }

        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) -> std::io::Result<()> {
        create_qdisc();

        loop {
            let mut buf = [0; 5];
            let result = stream.read(&mut buf);

            match result {
                Ok(size) => {
                    if size == 0 {
                        break;
                    }

                    let req: LimitReq = bincode::deserialize(buf.as_slice()).unwrap();

                    if req.state {
                        add_filter(req.limit.to_port());
                    } else {
                        delete_filter(req.limit.to_port());
                    }
                }
                Err(e) => {
                    println!("failed to read data: {}", e);
                }
            }
        }

        destroy_qdisc();

        Ok(())
    }
}
