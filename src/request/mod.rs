

use std::{
    collections::HashMap,
    io::Read,
    net::TcpStream, fmt::Display,
};

use crate::constant::{DIVER,CR};

pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
impl Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut header_str = String::from("{");
        for (key, value) in self.headers.iter() {
            header_str = format!("{}{}:\"{}\",", header_str, key, value);
        }
        header_str = format!("{}{}", header_str, "}");
        let str = format!(
            "method:{},uri:{},version:{},headers:{},body:{}",
            self.method, self.uri, self.version, header_str, self.body
        );
        write!(f, "{}", str)
    }
}


pub fn parse_request(stream: &mut TcpStream) -> Result<Request, String> {
    // 默认一个长度
    let mut buffer = [0; 1024];
    // 读取
    let read_result = stream.read(&mut buffer);
    return match read_result {
        Ok(_size) => {
            let mut index = 0;
            let length = buffer.len();
            let mut diver = 0;
            let mut content_length = 0;
            // 循环寻找header与body之前的分割符 按规范来说应该是\r\n\r\n
            while index < length - 4 {
                if DIVER
                    == &[
                        buffer[index],
                        buffer[index + 1],
                        buffer[index + 2],
                        buffer[index + 3],
                    ]
                {
                    diver = index;
                    break;
                } else {
                    //没获取到就继续获取
                    if buffer[index + 1] == CR[0] {
                        index += 1;
                    } else if buffer[index + 2] == CR[0] {
                        index += 2
                    } else if buffer[index + 3] == CR[0] {
                        index += 3
                    } else {
                        index += 4
                    }
                }
            }
            // 获取到以后的操作
            if diver != 0 {
                // 截取header部分
                let headers_cow = String::from_utf8_lossy(&buffer[0..diver]);
                let mut headers_lines = headers_cow.lines();
                let mut is_end = false;
                let mut headers = HashMap::<String, String>::new();
                let mut method = "";
                let mut uri = "";
                let mut version = "";
                let mut body: &[u8] = &[];

                // 遍历header
                while !is_end {
                    let line_result = headers_lines.next();
                    if let Some(line) = line_result {
                        if line.contains(": ") {
                            let split = line.split_once(": ");
                            if let Some((header_key, header_value)) = split {
                                let key = header_key.trim();
                                let value = header_value.trim();
                                if !key.is_empty() && !value.is_empty() {
                                    headers.insert(key.to_string(), value.to_string());
                                    if key == "Content-Length" {
                                        let num_length = value.trim().parse::<i64>();
                                        // 将其解析为i64
                                        if let Ok(length) = num_length {
                                            content_length = length;
                                        };
                                    }
                                }
                            }
                        } else {
                            let mut split = line.split_whitespace();
                            let method_option = split.next();
                            let uri_option = split.next();
                            let version_option = split.next();
                            if let Some(request_method) = method_option {
                                method = request_method;
                            }
                            if let Some(request_uri) = uri_option {
                                uri = request_uri;
                            }
                            if let Some(request_version) = version_option {
                                version = request_version;
                            }
                        }
                    } else {
                        is_end = true
                    }
                }
                let other_data_length: i64 = content_length - (1024 - diver as i64) + 4;
                // 如果需要继续获取
                if other_data_length > 0 {
                    // 新建一个向量 补足缺的长度
                    let mut other_buffer = vec![0; other_data_length as usize];
                    // 继续读取
                    let other_read_result = stream.read(&mut other_buffer);
                    if let Ok(_size) = other_read_result {
                        // 获取之前已经读取了一部分的body
                        body = &buffer[diver..];
                    }
                } else {
                    // 如果1024已经满足空间需求 直接获取body并打印
                    body = &buffer[diver..];
                }
                return Ok(Request {
                    method: String::from(method),
                    uri: String::from(uri),
                    headers,
                    version: String::from(version),
                    body: String::from_utf8_lossy(body).trim().to_string(),
                });
            } else {
                return Err(String::from("cant find diver"));
            }
            // 计算剩余还需要多大的长度

            // Ok(size)
        }
        Err(e) => Err(e.to_string()),
    };
    // return request;
}
