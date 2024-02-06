use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
};

use crate::ThreadPool;

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn from(method: &str) -> Self {
        match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => panic!("invalid request method"),
        }
    }
}

pub enum HttpVersion {
    HTTP1_1,
}

impl HttpVersion {
    pub fn to_string(&self) -> String {
        match self {
            HttpVersion::HTTP1_1 => "HTTP/1.1".to_string(),
        }
    }
}

pub enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    pub fn to_string(&self) -> String {
        match self {
            HttpStatus::Ok => "200 OK".to_string(),
            HttpStatus::NotFound => "404 NOT FOUND".to_string(),
        }
    }
}

pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
}

impl<'a> HttpRequest<'a> {
    pub fn from(request_line: &'a str) -> Self {
        let parts: Vec<&str> = request_line.split_whitespace().collect();

        if parts.len() != 3 {
            panic!("invalid request line");
        }

        Self {
            method: HttpMethod::from(parts[0]),
            path: parts[1],
        }
    }
}

pub struct HttpResponse<'a> {
    version: HttpVersion,
    status: HttpStatus,
    content: &'a str,
}

impl<'a> HttpResponse<'a> {
    pub fn new(version: HttpVersion, status: HttpStatus, content: &'a str) -> Self {
        Self {
            version,
            status,
            content,
        }
    }

    pub fn to_string(&self) -> String {
        let status_line = format!("{} {}", self.version.to_string(), self.status.to_string());
        let length = self.content.len();

        format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line, length, self.content
        )
    }
}

/// A simple HTTP client.
/// The `HttpClient` can be used to handle incoming connections and
/// send responses to the client.
pub struct HttpClient {
    thread_pool: Option<ThreadPool>,
}

impl HttpClient {
    /// Create a new `HttpClient` with a default thread pool size of zero.
    /// this means that the `HttpClient` will handle incoming connections
    /// in the main thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use webserver::HttpClient;
    ///
    /// let client = HttpClient::new();
    /// ```
    pub fn new() -> Self {
        HttpClient { thread_pool: None }
    }

    /// Create a new `HttpClient` with a thread pool of the given size.
    /// This means that the `HttpClient` will handle incoming connections
    /// in a separate thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use webserver::HttpClient;
    ///
    /// let client = HttpClient::with_threads(4);
    /// ```
    ///
    /// # Panics
    ///
    /// The `size` argument must be greater than zero.
    ///
    /// ```should_panic
    /// use webserver::HttpClient;
    ///
    /// let client = HttpClient::with_threads(0);
    /// ```
    pub fn with_threads(size: usize) -> Self {
        HttpClient {
            thread_pool: Some(ThreadPool::new(size)),
        }
    }

    /// Handle an incoming connection.
    /// If the `HttpClient` was created with a thread pool, the connection
    /// will be handled in a separate thread. Otherwise, the connection will
    /// be handled in the main thread.
    pub fn handle(&self, stream: TcpStream) {
        match &self.thread_pool {
            Some(pool) => pool.execute(|| HttpClient::process(stream)),
            None => HttpClient::process(stream),
        }
    }

    fn process(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let request = HttpRequest::from(request_line.as_str());

        let content = match fs::read_to_string(format!(
            "assets{}.html",
            match request.path {
                "/" => "/index",
                _ => request.path,
            }
        )) {
            Ok(content) => content,
            Err(_) => match fs::read_to_string("assets/404.html") {
                Ok(content) => content,
                Err(_) => String::from("404 Not Found"),
            },
        };

        let response = HttpResponse::new(HttpVersion::HTTP1_1, HttpStatus::Ok, &content);

        stream.write_all(response.to_string().as_bytes()).unwrap();
    }
}
