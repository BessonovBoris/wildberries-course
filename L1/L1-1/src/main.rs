trait Action {
    // use link to avoid moving value
    fn say(&self);
}

struct Person {
    name: String,
}

// trait realisation
impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let man = Person { name: "Petr".to_string() };
    man.say();
}
