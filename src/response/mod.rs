use crate::constant::StatusCode;



pub fn strify_response(code: StatusCode, body: &str) -> String {

    let status_line = format!("HTTP/1.1 {}\r\n\r\n", code);
    let response = format!("{}{}", status_line, body);
    return response;
}
