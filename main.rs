use std::{io::{BufRead, Write}, path::Path};
use urlencoding::encode;

fn main() {
    let listener: std::net::TcpListener = std::net::TcpListener::bind("127.0.0.1:9999").unwrap();
    for mut stream in listener.incoming().flatten() {
        let mut rdr: std::io::BufReader<&mut std::net::TcpStream> = std::io::BufReader::new(&mut stream);
        let mut l: String = String::new();
        rdr.read_line(&mut l).unwrap();
        match l.trim().split(' ').collect::<Vec<_>>().as_slice() {
            ["GET", resource, "HTTP/1.1"] => {
                loop {
                    let mut l: String = String::new();
                    rdr.read_line(&mut l).unwrap();
                    if l.trim().is_empty() { break; }
                    println!("{l}");
                }
                let mut p: std::path::PathBuf = std::path::PathBuf::new();
                p.push("website");
                println!("{}",resource.to_owned().to_string());
                if resource != &"/" { p.push(resource.trim_start_matches("/")); };
                if resource.contains('?') { p = std::path::PathBuf::from("website".to_string()+(&resource.split('?').collect::<Vec<&str>>()[0].to_owned().to_string())); };
                if !resource.ends_with('/') && !resource.contains('.') { let n: String = p.to_str().unwrap().to_owned(); p = std::path::PathBuf::from(n+"/index.html"); };
                if resource.ends_with("/") { p.push("index.html") };
                println!("{}",p.clone().into_os_string().into_string().unwrap());
                if Path::new(&p).exists() {
                    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
                    if resource.ends_with(".svg") {
                        stream.write_all(b"data:image/svg+xml;utf8,").unwrap();
                        stream.write_all(encode(&std::fs::read_to_string(p).unwrap()).into_owned().trim().as_bytes()).unwrap();
                    } else {
                        stream.write_all(&std::fs::read(p).unwrap()).unwrap();
                    }
                } else {
                    stream.write_all(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
                }
            }
            _ => todo!()
        }     
    }
}
