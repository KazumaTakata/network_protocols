use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::str;
use sha1::{Sha1, Digest};
use std::collections::HashMap;
use base64::{encode};
use std::convert::TryFrom;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8887").unwrap();
    loop {
        match listener.accept() {
            Ok((mut _socket, addr)) => {
                println!("new client: {:?}", addr);
                let mut buf = [0; 1024];
                _socket.read(&mut buf);

                let buf_str = str::from_utf8(&buf).unwrap();
                //println!("{}", buf_str);

                let mut header_map = HashMap::new();
                
                let headers:Vec<&str> =  buf_str.split("\r\n").collect();
                for header in headers {
                    let one_header:Vec<&str> = header.split(":").collect(); 
                    if one_header.len() > 1 {
                       header_map.insert(one_header[0], one_header[1]); 
                    }
                }
                match  header_map.get("Sec-WebSocket-Key") {
                    Some(key) =>
                    {
                        println!("{}", key.trim());
                        let mut trimed_key = key.trim().to_owned();
                        trimed_key.push_str("258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
         
                        let mut sh = Sha1::default();
                        sh.input(trimed_key);
                        let output = sh.result();
                        let encoded = encode(&output);
                        println!("{}", encoded); 
                        let b = format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade:websocket\r\nConnection:Upgrade\r\nSec-WebSocket-Accept:{}\r\n\r\n", encoded);
                        _socket.write(b.as_bytes());

                        let mut data = [0 ; 50]; 
                        while match _socket.read(&mut data) {
                            Ok(size) => {
                              /*  println!("{:#b}", &data[0]);*/
                                //println!("{:#b}", &data[1]);

                                //println!("{:#b}", &data[2]);
                                //println!("{:#b}", &data[3]);
                                //println!("{:#b}", &data[4]);
                                //println!("{:#b}", &data[5]);

                                //println!("{:#b}", &data[6]);
                                //println!("{:#b}", &data[7]);
                                //println!("{:#b}", &data[8]);
                                /*println!("{:#b}", &data[9]);*/
                            
                                let mut decoded = [0; 100];
                                let length = data[1] & 0b01111111;

                                for n in 0..length {
                                     let index = usize::try_from(n).unwrap();
                                     decoded[index] =  data[index % 4 + 2] ^ data[6 + index] ;
                                }

                                println!("{}",str::from_utf8(&decoded).unwrap());
                                
                                //_socket.write(&data);

                                

 
                                true
                            },
                            Err(_) => {
                                println!("An error occurred, terminating connection with {}", _socket.peer_addr().unwrap());
                                _socket.shutdown(Shutdown::Both).unwrap();
                                false
                            }
                         } {} 
                            

                        
                       
                    },
                    None => println!("no key")
                }
                    



           }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
} // the stream is closed here
