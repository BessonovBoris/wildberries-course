use clap::Parser;
use crate::{Command, FileSystem};

#[derive(Parser)]
pub struct Echo {
    #[clap(default_value = "")]
    line: String,

    #[clap(skip)]
    next: Option<Box<dyn Command>>,
}

impl Echo {
    pub fn from_input(input: &Vec<&str>) -> Echo {
        Echo::parse_from(input.iter())
    }
}

impl Command for Echo {
    fn execute(&mut self, file_system: &mut FileSystem) -> Result<Vec<String>, String> {
        if let Some(next) = self.next.as_mut() {
            next.execute(file_system)
        } else {
            Ok(vec![self.line.clone()])
        }
    }

    fn set_arg(&mut self, _args: Vec<String>) {}

    fn add_next(&mut self, command: Box<dyn Command>) {
        if let Some(next) = self.next.as_mut() {
            next.add_next(command);
            return;
        }

        self.next = Some(command);
    }
}