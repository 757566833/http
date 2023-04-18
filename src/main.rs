use std::{
    io::Write,
    net::{TcpListener, TcpStream}
};
mod constant;
mod pool;
mod request;
mod response;

use constant::StatusCode;
use request::parse_request;
use response::strify_response;
use pool::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let request_result = parse_request(&mut stream);
    if let Ok(request) = request_result {
        let response;
        if request.uri == "/" {
            response = strify_response(StatusCode::OK, "hello word");
        } else {
            response = strify_response(StatusCode::NotFound, "");
        }
        let _ = stream.write_all(response.as_bytes());
    } else {
        let response = strify_response(StatusCode::BadRequest, "");
        let _ = stream.write_all(response.as_bytes());
    }
}
fn main() {
    // 监听端口
    let listener_result = TcpListener::bind("127.0.0.1:8090");
    let listener = match listener_result {
        Ok(l) => l,
        Err(e) => {
            panic!("{}", e.to_string())
        }
    };
    let pool = ThreadPool::new(2);

    // 默认返回的response

    for stream_result in listener.incoming() {
        if let Ok(stream) = stream_result {
            pool.execute(||{
                handle_connection(stream);
            })
        }
    }
}
