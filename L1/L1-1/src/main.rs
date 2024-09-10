trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}

fn main() {
    let man = Person { name: "Petr".to_string() };
    man.say();
}
