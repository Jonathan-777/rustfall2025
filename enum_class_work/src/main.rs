#[derive(PartialEq, Debug)]
enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        for f in &self.fruit {
            print!("{:?}: ", f);
            Self::tell_me_joke(f);
        }
    }

    fn tell_me_joke(fruit: &Fruit) {
        match fruit {
            Fruit::Apple(msg) => println!("{}", msg),
            Fruit::Banana(msg) => println!("{}", msg),
            Fruit::Tomato(msg) => println!("{}", msg),
        }
    }
}

fn main() {
 let a = "Crunchy, sweet, portable.".to_string();
    let b = "Curvy, yellow, potassium power.".to_string();
    let t = "Technically a fruit, debatably delish.".to_string();

    let inventory = Inventory {
        fruit: vec![
            Fruit::Apple(a),
            Fruit::Banana(b),
            Fruit::Tomato(t),
        ],
    };

    inventory.available_fruits();
}
