use std::net::{TcpListener,TcpStream};
use std::io::{BufReader, BufRead,Write};
use std::fs;

fn handle_requests(stream:&mut TcpStream)->Vec<String>{
    let  buf_reader = BufReader::new(&mut *stream);
    let mut http_request = vec![];
    for line in buf_reader.lines(){
        match line {
            Ok(l)=>{
                if l.is_empty(){
                    break; 
                }
                else {
                    http_request.push(l);
                }
            },
            Err(e)=>{
                println!("error in reading line {}",e);
                break;
            }
        }
    }
     let html_file = match fs::read_to_string("index.html") {
        Ok(f) => f,
        Err(_) => {
            println!("index.html not found, using fallback page.");
            String::from("<h1>404 - index.html not found</h1>")
        }
    };
      let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
        html_file
    );
 
    match stream.write_all(response.as_bytes()){
        Ok(s)=>s,
        Err(e)=>{
            println!("error sending response {:?}",e);
        }
    }

    return http_request;
}
fn main() {
    // Try to bind the listener
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to bind: {}", e);
            return;
        }
    };

    //iterating over stream comming from listner which are the req by browser
 for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => {
                println!("Connection failed");
                continue; 
            }
        };
        let requests = handle_requests(&mut stream);
        println!("Client connected: {:?}", requests);
    }

    println!("Listener created: {:?}", listener);
}
