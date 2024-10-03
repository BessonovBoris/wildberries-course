use std::io::Write;
use crate::*;

pub struct Desktop;

impl Desktop {
    pub fn new() -> Self {
        Desktop {}
    }

    pub fn run(&self) {
        let path = Desktop::input();
        let mut file_system = FileSystem::from_input(&path);

        loop {
            print!("{}: ", file_system.path().to_string_lossy()[4..].to_string());

            let input = Desktop::input();
            let input = Desktop::parse_input(&input);
            let mut command: Box<dyn Command> = Box::new(BaseCommand::new());

            if input[0][0].eq("\\quit") {
                break;
            }

            for line in input {
                if line[0].eq("cd") {
                    let new_command = Box::new(ChangeDirectory::from_input(&line));
                    command.add_next(new_command);
                } else if line[0].eq("ls") {
                    let new_command = Box::new(List::from_input(&line));
                    command.add_next(new_command);
                } else if line[0].eq("pwd") {
                    let new_command = Box::new(PresentWorkingDirectory::new());
                    command.add_next(new_command);
                } else if line[0].eq("echo") {
                    let new_command = Box::new(Echo::from_input(&line));
                    command.add_next(new_command);
                } else if line[0].eq("ps") {
                    let new_command = Box::new(ProcessStatus::from_input(&line));
                    command.add_next(new_command);
                }
            }

            file_system.execute(command).unwrap_or_else(|err| {
                eprintln!("{}", err)
            });
        }
    }

    fn parse_input(input: &str) -> Vec<Vec<&str>> {
        let input = input
            .trim()
            .split('|')
            .map(|s| s.trim().split(' ').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        input
    }

    fn input() -> String {
        let mut input = String::new();

        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read stdin");
        let input = input.trim();

        input.to_string()
    }
}