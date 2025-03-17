use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use tiny_http::{Header, Request, Response, Server};

fn main() -> io::Result<()> {
    let server = Server::http("127.0.0.1:8080").unwrap();
    println!("Server listening on http://127.0.0.1:8080/");
    for request in server.incoming_requests() {
        handle_request(request)?;
    }
    Ok(())
}

fn handle_request(request: Request) -> io::Result<()> {
    let url = request.url();
    let path = Path::new(&url[1..]);
    println!("Request path: {}", url);

    // Handle /rollouts/ directory listing
    if url == "/rollouts/" {
        let rollouts_dir = PathBuf::from("./static/rollouts");
        let entries = fs::read_dir(&rollouts_dir)?;
        let files: Vec<String> = entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    if e.path().is_file() {
                        e.file_name().into_string().ok()
                    } else {
                        None
                    }
                })
            })
            .collect();

        let json = serde_json::to_string(&files).unwrap_or_else(|_| "[]".to_string());
        let response = Response::from_string(json)
            .with_header(Header::from_bytes(&b"Content-Type"[..], b"application/json").unwrap());
        return request
            .respond(response)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e));
    }

    let file_path = if path.to_str().unwrap_or_default() == "" {
        PathBuf::from("./static/index.html")
    } else if url.starts_with("/wasm/") {
        PathBuf::from("./survivor-wasm/pkg/").join(path.strip_prefix("wasm/").unwrap())
    } else if url.starts_with("/rollouts/") {
        // Dedicated route for rollout files
        PathBuf::from("./static/rollouts/").join(path.strip_prefix("rollouts/").unwrap())
    } else {
        PathBuf::from("./static/").join(path)
    };

    // Get and log the absolute file path
    let absolute_path = file_path
        .canonicalize()
        .unwrap_or_else(|_| file_path.clone());
    println!("Mapped to file path: {:?}", file_path);
    println!("Absolute file path: {:?}", absolute_path);

    if file_path.is_file() {
        let mut file = File::open(&file_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let mime_type = if file_path.extension().map_or(false, |ext| ext == "wasm") {
            "application/wasm"
        } else if file_path.extension().map_or(false, |ext| ext == "html") {
            "text/html; charset=utf-8"
        } else if file_path.extension().map_or(false, |ext| ext == "js") {
            "application/javascript"
        } else if file_path.extension().map_or(false, |ext| ext == "json") {
            "application/json"
        } else {
            "application/octet-stream"
        };
        let response = Response::from_data(contents).with_header(
            Header::from_bytes(&b"Content-Type"[..], mime_type.as_bytes().to_vec()).unwrap(),
        );
        request.respond(response)?;
    } else {
        println!("File not found: {:?}", file_path);
        request.respond(Response::from_string("404 Not Found").with_status_code(404))?;
    }
    Ok(())
}
