use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
mod version2;

fn handle_requests(stream: &mut TcpStream) -> Vec<String> {
     // We wrap the incoming TCP stream inside a BufReader.
    // BufReader buffers reads from the stream, so instead of reading byte by byte,
    // it reads a chunk into memory and lets us process it line by line efficiently.
    let buf_reader = BufReader::new(&mut *stream);

     // This vector will store each line of the HTTP request headers we read.
    let mut http_request = vec![];

    // Read the request line by line from the client (browser).
    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                if l.is_empty() {
                    break;
                } else {
                    http_request.push(l);
                }
            }
            Err(e) => {
                println!("error in reading line {}", e);
                break;
            }
        }
    }

    thread::sleep(Duration::from_secs(10));

    //loading html file to return as msg to the req
    let html_file = match fs::read_to_string("index.html") {
        Ok(f) => f,
        Err(_) => {
            println!("index.html not found, using fallback page.");
            String::from("<h1>404 - index.html not found</h1>")
        }
    };

    // creating the message where msg is "version status_code status_message  clrf and then out html message"
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}",
        html_file
    );

    // writing back response to browser
    match stream.write_all(response.as_bytes()) {
        Ok(s) => s,
        Err(e) => {
            println!("error sending response {:?}", e);
        }
    }

    return http_request;
}
fn main() {
    // Try to bind the listener
    // let listener = match TcpListener::bind("127.0.0.1:7878") {
    //     Ok(l) => l,
    //     Err(e) => {
    //         println!("Failed to bind: {}", e);
    //         return;
    //     }
    // };

    // this is for creating channel for incomming jobs
   // let (sender, reciever) = mpsc::channel::<TcpStream>();

    //we wrap the reciever into Arc so that it can be accessed accross all the threads and Mutex so that one thread can access the data at a single point of time . without mutex it can cause race condition .
   // let reciever = Arc::new(Mutex::new(reciever));

    // for i in 0..4 {
    //     //here each worker get a copy of reciever
    //     let reciever = Arc::clone(&reciever);
    //     //we used move here so that i can use reciever inside of the clouser in thread spawn
    //     thread::spawn(move || {
    //         loop {
    //             match reciever.lock() {
    //                 Ok(r) => match r.recv() {
    //                     Ok(mut stream) => {
    //                         println!("Worker got a job {i}");
    //                         handle_requests(&mut stream);
    //                     }
    //                     Err(e) => {
    //                         println!("Error receiving job: {:?}", e);
    //                         break; // stop worker when channel is closed
    //                     }
    //                 },
    //                 Err(e) => {
    //                     println!("Error locking receiver: {:?}", e);
    //                     break; // stop worker if mutex is poisoned
    //                 }
    //             }
    //         }
    //     });
    // }
    //iterating over stream comming from listner which are the req by browser and this is single thread way of doing http server
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(s) => {
    //             if let Err(e) = sender.send(s) {
    //                 println!("Failed to send job to worker: {:?}", e);
    //             }
    //         }
    //         Err(e) => {
    //             println!("Connection failed: {:?}", e);
    //             continue;
    //         }
    //     }
    // }

    // println!("Listener created: {:?}", listener);

    //in this version i used thread pool created a thread pool and imp it 
    version2::wrapper();
}
