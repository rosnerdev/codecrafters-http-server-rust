use std::io::{BufReader, Read, Write};
use std::net::TcpListener;
use std::thread;
use std::env;
use std::fs::{self, File};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        thread::spawn(move || {
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut buffer = [0; 1024];

            reader.read(&mut buffer).unwrap();

            let req_str = String::from_utf8_lossy(&buffer[..]);

            let path = req_str.split(" ").nth(1);
            let args: Vec<String> = env::args().collect();

            match path {
                Some(path) => {
                    if path == "/" {
                        stream
                                .write_all("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n".as_bytes())
                                .unwrap();
                        stream.flush().unwrap();
                    } else if path.starts_with("/echo/") {
                        let param = if path.len() > 6 { &path[6..] } else { "" };

                        stream
                                .write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", param.len(), param).as_bytes())
                                .unwrap();
                        stream.flush().unwrap();
                    } else if path == "/user-agent" {
                        let user_agent = req_str.split("\r\n").nth(2).unwrap();
                        let agent = user_agent.split(": ").nth(1).unwrap();

                        stream
                                .write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", agent.len(), agent).as_bytes())
                                .unwrap();
                        stream.flush().unwrap();
                    } else if path.starts_with("/files/") && path.len() > 7 && args.len() == 3 && args.get(1) == Some(&String::from("--directory")) {
                        let file_str = if path.len() > 7 { &path[7..] } else { "" };
                        let dir_str = match args.get(2) {
                            Some(dir) => {dir}
                            None => {""}
                        };

                        let file_path = format!("{}/{}", dir_str, file_str);
                        println!("{file_path}");
                        let metadata_result = fs::metadata(&file_path);

                        if metadata_result.is_ok() {
                            let mut file = File::open(file_path).unwrap();
                            let mut contents = String::new();
                            file.read_to_string(&mut contents).unwrap();

                            stream
                                .write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/octet-stream\r\nContent-Length: {}\r\n\r\n{}", contents.len(), contents).as_bytes())
                                .unwrap();
                            stream.flush().unwrap();
                        } else {
                            stream
                                .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                                .unwrap();
                            stream.flush().unwrap();
                        }
                    } else {
                        stream
                            .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                            .unwrap();
                        stream.flush().unwrap();
                    }
                }
                None => {}
            }
        });
    }
}
