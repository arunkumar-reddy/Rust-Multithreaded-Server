use std::fs;
use crate::server_core::request::Request;
use crate::server_core::response::Response;
use crate::server_core::response::ContentType;

const ROOT_DIR: &str = "public";

pub fn handle_connection(request: Request, response: Response) {
    let method = request.method;
    let path = request.path;
    match method.as_str() {
        "GET" => handle_get_request(path, response),
        _ => handle_other_request(response)
    }
}

pub fn handle_get_request(path: String, mut response: Response) {
    let file_path = match path.as_str() {
        "/" => format!("{}{}", ROOT_DIR, "/index.html"),
        _ => format!("{}{}", ROOT_DIR, path)
    };
    let ext = file_path.split(".").last(); 
    match ext  {
        Some(extension) => {
            let file_content = fs::read(&file_path);
            match file_content {
                Ok(content) => {
                    response.status_code = String::from("200");
                    response.body = Some(content);
                    response.headers.content_type = ContentType::get_content_type(extension);
                    response.send();
                },
                Err(_) => handle_not_found(response)
            }
        },
        None => handle_not_found(response)
    }
}

pub fn handle_not_found(mut response: Response) {
    let file_path = format!("{}{}", ROOT_DIR, "/404.html");
    let file_content = fs::read(file_path);
    match file_content {
        Ok(content) => {
            response.status_code = String::from("404");
            response.body = Some(content);
            response.headers.content_type = ContentType::HTML;
            response.send();
        },
        Err(error) => panic!("Failed to read 404.html: {:?}", error)
    }
}

pub fn handle_other_request(mut response: Response) {
    let file_path = format!("{}{}", ROOT_DIR, "/405.html");
    let file_content = fs::read(file_path);
    match file_content {
        Ok(content) => {
            response.status_code = String::from("405");
            response.body = Some(content);
            response.headers.content_type = ContentType::HTML;
            response.send();
        },
        Err(error) => panic!("Failed to read 405.html: {:?}", error)
    }
}