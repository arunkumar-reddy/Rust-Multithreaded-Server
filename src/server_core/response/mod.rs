use std::io::prelude::*;
use std::net::TcpStream;

pub enum ContentType {
    CSS,
    GIF,
    HTML,
    JPEG,
    PNG,
    SVG,
    TEXT,
    XML,
}

impl ContentType {
    pub fn get_content_type(ext: &str) -> ContentType {
        match ext {
            "css" => ContentType::CSS,
            "gif" => ContentType::GIF,
            "htm" => ContentType::HTML,
            "html" => ContentType::HTML,
            "jpeg" => ContentType::JPEG,
            "jpg" => ContentType::JPEG,
            "png" => ContentType::PNG,
            "svg" => ContentType::SVG,
            "txt" => ContentType::TEXT,
            "xml" => ContentType::XML,
            _ => ContentType::TEXT,
        }
    }

    fn value(&self) -> &str {
        match *self {
            ContentType::CSS => "text/css",
            ContentType::GIF => "image/gif",
            ContentType::HTML => "text/html",
            ContentType::JPEG => "image/jpeg",
            ContentType::PNG => "image/png",
            ContentType::SVG => "image/svg+xml",
            ContentType::TEXT => "text/plain",
            ContentType::XML => "application/xml",
        }
    }
}

pub struct ResponseHeaders {
    pub content_type: ContentType
}

impl ResponseHeaders {
    pub fn new(content_type: ContentType) -> ResponseHeaders {
        ResponseHeaders { content_type }
    }
}

pub struct Response {
    pub body: Option<Vec<u8> >,
    pub status_code: String,
    pub headers: ResponseHeaders,
    stream: TcpStream
}

impl Response {
    pub fn new(status_code: String, headers: ResponseHeaders, body: Option<Vec<u8> >, stream: TcpStream) -> Response {
        Response { status_code, headers, body, stream }
    }

    pub fn send(&mut self) {
        let result = match self.status_code.as_str() {
            "200" => "HTTP/1.1 200 OK\r\n",
            "404" => "HTTP/1.1 404 NOT FOUND\r\n",
            "405" => "HTTP/1.1 405 NOT SUPPORTED\r\n",
            _ => "HTTP/1.1 404 NOT FOUND\r\n",
        };
        let result = format!("{} Content-type: {}\n", result, self.headers.content_type.value());
        let mut bytes = result.as_bytes().to_vec();
        match &mut self.body {
            Some(body) => {
                bytes.append(&mut "\n".as_bytes().to_vec());
                bytes.append(body);
            },
            None => () 
        }
        match self.stream.write(&bytes) {
            Ok(_) => {
                match self.stream.flush() {
                    Ok(_) => {},
                    Err(error) => panic!("Failed to flush stream with error: {:?}", error)
                }
            },
            Err(error) => panic!("Failed to write to stream with error: {:?}", error)
        }
    }
}