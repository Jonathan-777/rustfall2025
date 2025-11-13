
trait ShowInfo {
    fn show_info(&self);
}

struct UndergradStudent {
    name: String,
    major: String,
    gpa: f32,
}

struct GradStudent {
    name: String,
    major: String,
    gpa: f32,
    thesis_title: String,
}

impl ShowInfo for UndergradStudent {
    fn show_info(&self) {
        println!(
            "Undergrad: {} | Major: {} | GPA: {:.2}",
            self.name, self.major, self.gpa
        );
    }
}

impl ShowInfo for GradStudent {
    fn show_info(&self) {
        println!(
            "Grad: {} | Major: {} | GPA: {:.2} | Thesis: \"{}\"",
            self.name, self.major, self.gpa, self.thesis_title
        );
    }
}


struct Enrollment {
    students: Vec<Box<dyn ShowInfo>>,
}

impl Enrollment {
    fn new() -> Self {
        Self {
            students: Vec::new(),
        }
    }

    
    fn enroll<S>(&mut self, student: S)
    where
        S: ShowInfo + 'static,
    {
        self.students.push(Box::new(student));
    }
}


impl ShowInfo for Enrollment {
    fn show_info(&self) {
        println!("--- Enrollment ---");
        for student in &self.students {
            student.show_info();
        }
    }
}


fn print_student_info(student: &impl ShowInfo) {
     student.show_info();
}

fn main() {
    let alice = UndergradStudent {
        name: "Alice".to_string(),
        major: "Computer Science".to_string(),
        gpa: 3.75,
    };

    let bob = GradStudent {
        name: "Bob".to_string(), 
        major: "Data Science".to_string(),
        gpa: 3.9,
        thesis_title: "Rust for High-Performance Systems".to_string(),
    };

    let mut enrollment = Enrollment::new();
    enrollment.enroll(alice);
    enrollment.enroll(bob);

    
    print_student_info(&enrollment); 
}

