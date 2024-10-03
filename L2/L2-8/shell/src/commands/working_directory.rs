use crate::Command;
use crate::file_system::FileSystem;

pub struct PresentWorkingDirectory {
    next: Option<Box<dyn Command>>,
}

impl PresentWorkingDirectory {
    pub fn new() -> PresentWorkingDirectory {
        PresentWorkingDirectory { next: None }
    }
}

impl Command for PresentWorkingDirectory {
    fn execute(&mut self, file_system: &mut FileSystem) -> Result<Vec<String>, String> {
        let output = vec![file_system.path().to_string_lossy().into_owned()];

        if let Some(next) = self.next.as_mut() {
            next.set_arg(output);
            next.execute(file_system)
        } else {
            Ok(output)
        }
    }

    fn set_arg(&mut self, _args: Vec<String>) {
    }

    fn add_next(&mut self, command: Box<dyn Command>) {
        if let Some(next) = self.next.as_mut() {
            next.add_next(command);
            return;
        }

        self.next = Some(command);
    }
}