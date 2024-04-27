pub fn get_current_time() -> u128 {
    0
}

pub fn parse_response(data: Result<String, String>) -> String {
    match data {
        Ok(d) => format!(r#"{{"type":"success","data":{d}}}"#),
        Err(e) => format!(r#"{{"type":"fail","error":{e}}}"#)
    }
}

pub fn parse_response_to_string<T: serde::Serialize>(data: Result<T, T>) -> String {
    match data {
        Ok(d) => format!(r#"{{"type":"success","data":"{}"}}"#, urlencoding::encode(serde_json::to_string(&d).unwrap().as_str()).to_string()),
        Err(e) => format!(r#"{{"type":"fail","error":"{}"}}"#, urlencoding::encode(serde_json::to_string(&e).unwrap().as_str()).to_string())
    }
}
