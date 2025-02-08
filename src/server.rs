use std::{collections::HashMap, fmt::Error, net::{TcpListener, TcpStream}};
use std::io::prelude::*;

use crate::serializer::{Serializer, Value};

pub struct Server {
    host: String,
    port: String,
    serializer: Serializer,
    store: HashMap<String, Value>,
}

pub struct Response {}

pub struct Request {}

impl Server {
    pub fn new(host: String, port: String) -> Result<Self, std::io::Error> {
        let mut new_hashmap = HashMap::<String, Value>::new();
        return Ok(Server {
            host: host,
            port: port,
            serializer: Serializer::new(),
            store: new_hashmap,
        });
    }

    fn handle_incoming_value(&mut self, incoming_value: &Value) -> String {
        // return self.serializer.serialize(&Value::String(String::from("Pong")));
        match incoming_value {
            Value::Array(internal_ary) => {
                println!("internal ary is {:?}", internal_ary);
                match &internal_ary[0] {
                    Value::Bulk(internal_string) => {
                        let lower_string = internal_string.to_lowercase();
                        if  lower_string == "ping" {
                            return self.serializer.serialize(&Value::String(String::from("PONG")));
                        } else if lower_string == "echo" {
                            if let Value::Bulk(second_string) = &internal_ary[1] {
                                return self.serializer.serialize(&Value::String(String::from(second_string)));
                            } else {
                                return self.serializer.serialize(&Value::String(String::from("Error")));
                            }
                        } else if lower_string == "set" {
                            if let Value::Bulk(second_string) = &internal_ary[1] {
                                self.store.insert(
                                    String::from(second_string),
                                    internal_ary[2].clone(),
                                );
                                return self.serializer.serialize(&Value::String(String::from("OK")));
                            } else {
                                return self.serializer.serialize(&Value::String(String::from("Error")));
                            }
                        } else if lower_string == "get" {
                            println!("we're here");
                            if let Value::Bulk(second_string) = &internal_ary[1] {
                                println!("second string is {:?}", second_string);
                                if let Some(value) = self.store.get(second_string) {
                                    return self.serializer.serialize(&value);
                                } else {
                                    return self.serializer.serialize(&Value::String(String::from("Error")));
                                }
                            } else {
                                return self.serializer.serialize(&Value::String(String::from("Error")));    
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

    fn handle_client(&mut self, stream: &mut TcpStream) -> std::io::Result<()> {
        let mut buf = [0; 512];
        let _ = stream.read(&mut buf);

        if let Ok(value) = std::str::from_utf8(&buf) {
            let aron = self.serializer.deserialize(value);
            let result = self.handle_incoming_value(&aron);
            let _ = stream.write(result.as_bytes());
        }
        return Ok(());
    }

    pub fn start(&mut self) -> std::io::Result<()> {
        let new_listener = TcpListener::bind(format!("{}:{}", self.host, self.port))?;
        for stream in new_listener.incoming() {
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