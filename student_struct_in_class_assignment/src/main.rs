struct Student {
    name: String,
    major: String,
}

impl Student {
     
    fn new(name: String, major: String) -> Self {
        Student { name, major }
    }

    // setter method
    fn set_major(&mut self, new_major: String) {
        self.major = new_major;
    }

    // getter method
    fn get_major(&self) -> &str {
        &self.major
    }
}
fn main() {
    let mut s = Student::new("Alice".to_string(), "Math".to_string());

    
    println!("{}'s major: {}", s.name, s.get_major());

    s.set_major("Computer Science".to_string());

    println!("{}'s updated major: {}", s.name, s.get_major());
}
