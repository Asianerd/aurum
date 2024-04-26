pub fn get_current_time() -> u128 {
    0
}

pub fn parse_response(data: Result<String, String>) -> String {
    match data {
        Ok(d) => format!(r#"{{"type":"success","data":{d}}}"#),
        Err(e) => format!(r#"{{"type":"fail","error":"{e}"}}"#)
    }
}
