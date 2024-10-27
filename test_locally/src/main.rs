use std::{
    fs::File,
    io::{Read, Write},
    net::TcpListener,
    thread,
};

const ADDRESS: &'static str = "127.0.0.1:42069";

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("listener");

    println!("Listening on {ADDRESS}");

    for mut stream in listener.incoming().filter_map(Result::ok) {
        thread::spawn(move || {
            let mut request = vec![0; 1024];
            let len = stream.read(&mut request).unwrap();

            let request: String = request[..len].into_iter().map(|x| *x as char).collect();
            let mut request = request.split_whitespace();

            assert_eq!(request.next().unwrap(), "GET");

            let mut path = request.next().unwrap();

            if path == "/" {
                path = "/index.html"
            }

            let path = format!("../{path}");

            dbg!(&path);

            if path.ends_with("html") {
                stream.write(b"HTTP/1.1 200 OK\n").unwrap();
                stream.write(b"Content-type:text/html\n").unwrap();
                stream.write(b"Connection: close\n\n").unwrap();
            } else if path.ends_with(".js") {
                stream.write(b"HTTP/1.1 200 OK\n").unwrap();
                stream
                    .write(b"Content-type:application/javascript\n")
                    .unwrap();
                stream.write(b"Connection: close\n\n").unwrap();
            } else if path.ends_with(".wasm") {
                stream.write(b"HTTP/1.1 200 OK\n").unwrap();
                stream.write(b"Content-type:application/wasm\n").unwrap();
                stream.write(b"Connection: close\n\n").unwrap();
            }

            let mut file = File::open(path).unwrap();
            let mut content = Vec::new();
            file.read_to_end(&mut content).unwrap();
            stream.write_all(&mut content).unwrap();

            stream.shutdown(std::net::Shutdown::Both)
        });
    }
}
