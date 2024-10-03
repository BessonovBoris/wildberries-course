use crate::{Command, FileSystem};

pub struct BaseCommand {
    next: Option<Box<dyn Command>>,
}

impl BaseCommand {
    pub fn new() -> BaseCommand {
        BaseCommand { next: None }
    }
}

impl Command for BaseCommand {
    fn execute(&mut self, file_system: &mut FileSystem) -> Result<Vec<String>, String> {
        if let Some(next) = self.next.as_mut() {
            next.execute(file_system)
        } else {
            Ok(vec![])
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