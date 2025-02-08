use std::{fmt::Error, net::{TcpListener, TcpStream}};
use std::io::prelude::*;

use crate::serializer::{Serializer, Value};

pub struct Server {
    host: String,
    port: String,
    listener: TcpListener,
    serializer: Serializer,
}

pub struct Response {}

pub struct Request {}

impl Server {
    pub fn new(host: String, port: String) -> Result<Self, std::io::Error> {
        let new_listener = TcpListener::bind(format!("{}:{}", host, port))?;
        
        return Ok(Server {
            host: host,
            port: port,
            listener: new_listener,
            serializer: Serializer::new(),
        });
    }

    fn handle_incoming_value(&self, incoming_value: &Value) -> String {
        // return self.serializer.serialize(&Value::String(String::from("Pong")));
        match incoming_value {
            Value::Array(internal_ary) => {
                println!("internal ary is {:?}", internal_ary);
                match &internal_ary[0] {
                    Value::Bulk(internal_string) => {
                        let lower_string = internal_string.to_lowercase();
                        if  lower_string == "ping" {
                            return String::from("PONG");
                        } else if lower_string == "echo" {
                            if let Value::Bulk(second_string) = &internal_ary[1] {
                            return String::from(second_string);
                            } else {
                                return String::from("Error");
                            }
                        } 
                    },
                    _ => {

                    }
                }
            },
            _ => {}
        }
        todo!()
    }

    fn handle_client(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        let mut buf = [0; 512];
        let _ = stream.read(&mut buf);

        if let Ok(value) = std::str::from_utf8(&buf) {
            let aron = self.serializer.deserialize(value);
            let result = self.handle_incoming_value(&aron);
            let _ = stream.write(result.as_bytes());
        }
        return Ok(());
    }

    pub fn start(&self) -> std::io::Result<()> {
        for stream in self.listener.incoming() {
            self.handle_client(&mut stream?);
        }
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn server_receives_ping() {
    //     let mut stream = TcpStream::connect("127.0.0.1:3000")?;

    //     // let string_to_send = String::from("*1\r\n$4\r\nping\r\n");
    //     let string_to_send = String::from("*2\r\n$4\r\necho\r\n$11\r\nhello world\r\n");
    //     stream.write(string_to_send.as_bytes())?;
    //     stream.read(&mut [0; 128])?;
    //     Ok(())
    // }
}