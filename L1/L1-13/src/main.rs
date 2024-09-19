use std::hash::{DefaultHasher, Hash, Hasher};

struct Set {
    data: Vec<Vec<String>>,
}

impl Set {
    fn new() -> Self {
        let mut vec = Vec::new();
        vec.resize_with(32, Default::default);

        Set { data: vec }
    }

    fn insert(&mut self, s: String) {
        if self.contains(&s) {
            return;
        }

        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let hash = hasher.finish() as usize % self.data.capacity();

        self.data[hash].push(s);
    }

    fn contains(&self, s: &String) -> bool {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        let hash = hasher.finish() as usize % self.data.capacity();
        self.data[hash].iter().any(|x| x == s)
    }
}

fn number_of_strings() -> Result<usize, String> {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string())
    }

    let num = match input.trim().parse::<usize>() {
        Ok(num) => num,
        Err(err) => return Err(err.to_string())
    };

    Ok(num)
}

fn read_string() -> Result<String, String> {
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string())
    }

    Ok(input.trim().to_string())
}

fn main() {
    let n = number_of_strings().unwrap();
    let mut set = Set::new();
    let mut ans = Vec::new();

    for _ in 0..n {
        let s = read_string().unwrap();
        if !set.contains(&s) {
            ans.push(s.clone());
        }

        set.insert(s);
    }

    for s in ans.iter() {
        println!("{}", s);
    }
}
