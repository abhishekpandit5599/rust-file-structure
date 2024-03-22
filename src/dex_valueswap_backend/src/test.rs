pub struct Example {
    pub number: i32,
}

impl Example {
    pub fn init(&mut self) -> bool {
        self.number = 1;
        return true;
    }
    pub fn whoami() -> String {
        return "I am Abhishek Sharma.".to_string();
    }
    pub fn increment(&mut self) -> i32 {
        self.number += 1;
        return self.number;
    }
    pub fn decrement(&mut self) -> i32 {
        self.number -= 1;
        return self.number;
    }
    pub fn get_value(&mut self) -> i32 {
        return self.number;
    }
    pub fn set_value(&mut self,number: i32) -> i32 {
        self.number = number;
        return self.number;
    }
}