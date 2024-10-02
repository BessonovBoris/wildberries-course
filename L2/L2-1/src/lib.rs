use std::io::BufRead;

pub struct Program {
    lines: Vec<String>,

    // optional functions
    count_words: Option<fn(&Program) -> usize>,
    count_lines: Option<fn(&Program) -> usize>,
    count_chars: Option<fn(&Program) -> usize>,
}

impl Program {
    pub fn new(lines: Vec<String>) -> Self {
        Program { lines, count_words: None, count_lines: None, count_chars: None }
    }

    pub fn with_file_name(file: String) -> Self {
        // open file and read it to buffer
        let file = std::io::BufReader::new(std::fs::File::open(file).unwrap());

        // read 'Ok' lines and collect them into Strings
        let s = file.lines().map_while(|ok| ok.ok()).collect::<Vec<String>>();

        Program::new(s)
    }

    pub fn add_count_words(&mut self) {
        if self.count_words.is_none() {
            self.count_words = Some(Program::count_words)
        }
    }

    pub fn add_count_lines(&mut self) {
        if self.count_lines.is_none() {
            self.count_lines = Some(Program::count_lines)
        }
    }

    pub fn add_count_chars(&mut self) {
        if self.count_chars.is_none() {
            self.count_chars = Some(Program::count_chars)
        }
    }

    pub fn run(&self) -> Vec<usize> {
        let mut output = vec![];

        if let Some(count) = self.count_lines {
            output.push(count(self));
        }

        if let Some(count) = self.count_words {
            output.push(count(self));
        }

        if let Some(count) = self.count_chars {
            output.push(count(self));
        }

        output
    }

    // functions
    fn count_words(&self) -> usize {
        let mut ans = 0;

        for line in self.lines.iter() {
            ans += line.split_whitespace().count();
        }

        ans
    }

    fn count_lines(&self) -> usize {
        self.lines.len()
    }

    fn count_chars(&self) -> usize {
        let mut ans = 0;
        for line in self.lines.iter() {
            ans += line.chars().count();
        }

        ans
    }
}