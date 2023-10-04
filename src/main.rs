use std::net::TcpListener;
use std::io::{BufReader, Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut buffer = [0; 1024];

                reader.read(&mut buffer).unwrap();

                let req_str = String::from_utf8_lossy(&buffer[..]);
                
                let path = req_str.split(" ").nth(1);
                match path {
                    Some(path) => {
                        if path == "/" {
                            stream
                                .write_all("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n".as_bytes())
                                .unwrap();
                            stream.flush().unwrap();
                        } else if path.starts_with("/echo/") {
                            let param = if path.len() > 6 {&path[6..]} else {""};

                            stream
                                .write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", param.len(), param).as_bytes())
                                .unwrap();
                            stream.flush().unwrap();
                        } else if path == "/user-agent" {
                            // let agent = req_str
                            //     .split("\r\n")
                            //     .find(|line| line.starts_with("User-Agent: "))
                            //     .map(|line| line.split(": ").collect::<Vec<&str>>()[1])
                            //     .unwrap();

                            let user_agent = req_str.split("\r\n").nth(2).unwrap();

                            let agent = user_agent.split(": ").nth(1).unwrap();

                            stream
                                .write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", agent.len(), agent).as_bytes())
                                .unwrap();
                            stream.flush().unwrap();
                        } else {
                            stream
                                .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                                .unwrap();
                            stream.flush().unwrap();
                        }
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
