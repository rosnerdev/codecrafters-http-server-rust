use std::net::TcpListener;
use std::io::{BufReader, BufRead, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut req_str = String::new();

                reader.read_line(&mut req_str).unwrap();
                
                let _path = req_str.split(" ").nth(1);
                match _path {
                    Some(path) => {
                        let param = path.split("/").nth(2);

                        match param {
                            Some(param) => {
                                stream
                                    .write_all(format!("HTTP/1.1 200 OK\r\n\r\nContent-Type: text/plain\r\n\r\nContent-Length: {}\r\n\r\n{}", param.len(), param).as_bytes())
                                    .unwrap();
                                stream.flush().unwrap();
                            }
                            None => {}
                        }
                        // if path != "/" {
                        //     let param = path.split("/").nth(2);

                        //     match param {
                        //         Some(param) => {
                        //             stream
                        //                 .write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", param.len(), param).as_bytes())
                        //                 .unwrap();
                        //             stream.flush().unwrap();
                        //         }
                        //         None => {}
                        //     }
                        // } else {
                        //     stream
                        //         .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                        //         .unwrap();
                        //     stream.flush().unwrap();
                        // }
                    }
                    None => {}
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
