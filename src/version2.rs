use std::sync::{Arc, Mutex, mpsc};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufReader, BufRead, Write};
use std::fs;
use std::time::Duration;

struct ThreadPool {
    workers: Vec<thread::JoinHandle<()>>,
    sender: mpsc::Sender<TcpStream>,
}

fn handle_requests(stream: &mut TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(&mut *stream);
    let mut http_request = vec![];

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

    thread::sleep(Duration::from_secs(1));

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

    if let Err(e) = stream.write_all(response.as_bytes()) {
        println!("error sending response {:?}", e);
    }

    http_request
}

impl ThreadPool {
    fn new(n: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel::<TcpStream>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(n);

        for i in 0..n {
            let receiver = Arc::clone(&receiver);
            let handle = thread::spawn(move || loop {
                let stream = {
                    let rx = receiver.lock().unwrap();
                    rx.recv()
                };

                match stream {
                    Ok(mut stream) => {
                        println!("Worker {i} got a job");
                        handle_requests(&mut stream);
                    }
                    Err(_) => {
                        println!("Worker {i} shutting down.");
                        break;
                    }
                }
            });

            workers.push(handle);
        }

        ThreadPool { workers, sender }
    }

    fn send_jobs(&self, stream: TcpStream) {
        if let Err(e) = self.sender.send(stream) {
            println!("Error sending job: {:?}", e);
        }
    }
}

pub fn wrapper() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(l) => l,
        Err(e) => {
            println!("Failed to bind: {}", e);
            return;
        }
    };

    println!("Server running on http://127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                pool.send_jobs(s);
            }
            Err(e) => {
                println!("Connection failed: {:?}", e);
            }
        }
    }
}
