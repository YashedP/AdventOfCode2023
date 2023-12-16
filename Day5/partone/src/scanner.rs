use std::io;

pub struct Scanner {
    line: String,
    already_read: bool,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            line: String::new(),
            already_read: false,
        }
    }

    pub fn has_next_line(&mut self) -> bool {
        if self.already_read {
            return true;
        } else {
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(n_bytes) if n_bytes > 0 => {
                    self.line = buffer.trim().to_string();
                    self.already_read = true;
                    true
                }
                Ok(_) | Err(_) => {
                    self.line = String::new();
                    self.already_read = true;
                    false
                }
            }                
        }
    }

    pub fn read(&mut self) -> &str {
        if !self.already_read {
            let mut buffer = String::new();
            match io::stdin().read_line(&mut buffer) {
                Ok(n_bytes) if n_bytes > 0 => {
                    self.line = buffer.trim().to_string();
                }
                Ok(_) | Err(_) => {
                    self.line = String::new();
                }
            }
        }
        self.already_read = false;
        &self.line
    }
}