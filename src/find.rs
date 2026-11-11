use std::fs;

pub fn read_f(file: &str) -> (String, bool) {
    match fs::read_to_string(&file) {
        Ok(link) => (link, true), 
        Err(e) => (e.to_string(), false)
    }
}
