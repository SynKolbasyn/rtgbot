pub struct Bot {
    token: String,
}


impl Bot {
    pub fn new<T: ToString>(token: T) -> Bot {
        Bot {
            token: token.to_string(),
        }
    }
    
    
    pub fn token(&self) -> String {
        self.token.clone()
    }
}
