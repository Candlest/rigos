use std::io::{BufReader, Write, BufRead, self, Read};

use crate::utils;
/*Listen TCP */
pub fn run_server(){
    let listener = std::net::TcpListener::bind("127.0.0.1:7878").unwrap();
    utils::info(utils::Info::RUN, "at", "http://localhost:7878");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
         std::thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

/*Tackle HTTP 
Read the uri of the HTTP request and return the matching file content:
For requests to /, index.html is returned
For file requests, return the content if it exists, or return 404.html if it does not exist
*/
fn handle_connection(mut stream: std::net::TcpStream){
    let buf_reader = std::io::BufReader::new(&mut stream);
    let request_line = std::io::BufRead::lines(buf_reader).next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "/index.html".to_string()),
        _ => {
            let url_phase: Vec<&str> = request_line[..].split(' ').collect();
            let url_raw = url_phase[1]; /*get uri */
            let iter = percent_encoding::percent_decode(url_raw.as_bytes());
            let decoded = iter.decode_utf8().unwrap().to_string();
            let path_str = format!("{}{}", utils::PUBLIC_DIR, decoded);
            let path_str = path_str.as_str();
            if std::path::Path::exists(std::path::Path::new(path_str)){
                ("HTTP/1.1 200 OK", decoded)
            }
            else {
                crate::utils::info(utils::Info::RUN, "cannot found", &decoded);
                ("HTTP/1.1 404 NOT FOUND", "/404.html".to_string())
            }
        }
    };

    let contents = std::fs::read_to_string(format!("{}{}", utils::PUBLIC_DIR, filename).as_str()).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    std::io::Write::write_all(&mut stream, response.as_bytes()).unwrap();
}