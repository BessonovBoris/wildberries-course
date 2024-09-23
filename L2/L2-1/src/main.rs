use std::io::BufRead;

struct Program {
    lines: Vec<String>,
}

impl Program {
    fn new(lines: Vec<String>) -> Self {
        Program { lines, }
    }

    fn with_file_name(file: String) -> Self {
        let file = std::io::BufReader::new(std::fs::File::open(file).unwrap());
        let s = file.lines().filter_map(|ok| ok.ok()).collect::<Vec<String>>();

        Program::new(s)
    }

    fn word_count(&self) -> usize {
        let mut ans = 0;

        for line in self.lines.iter() {
            ans += line.split_whitespace().count();
        }

        ans
    }

    fn lines_count(&self) -> usize {
        self.lines.len()
    }

    fn char_count(&self) -> usize {
        let mut ans = 0;
        for line in self.lines.iter() {
            ans += line.chars().count();
        }

        ans
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    let filename = args.last().unwrap().to_string();
    let program = Program::with_file_name(filename.clone());
    let mut nums = Vec::new();

    for i in 1..args.len()-1 {
        if args[i].eq("-c") {
            nums.push(program.char_count());
        } else if args[i].eq("-l") {
            nums.push(program.lines_count());
        } else if args[i].eq("-w") {
            nums.push(program.word_count());
        }
    }

    if args.len() == 2 {
        nums.push(program.word_count());
    }

    for num in nums.iter() {
        print!("{} ", num);
    }

    println!("{}", filename);
}
