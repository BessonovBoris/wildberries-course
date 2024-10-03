use std::path::PathBuf;
use clap::Parser;
use crate::Command;
use crate::file_system::FileSystem;

#[derive(Parser)]
pub struct ChangeDirectory {
    path: PathBuf,

    #[clap(skip)]
    next: Option<Box<dyn Command>>,
}

impl ChangeDirectory {
    pub fn from_input(input: &Vec<&str>) -> ChangeDirectory {
        ChangeDirectory::parse_from(input.iter())
    }
}

impl Command for ChangeDirectory {
    fn execute(&mut self, file_system: &mut FileSystem) -> Result<Vec<String>, String> {
        let path = file_system.path().join(&self.path);

        if !path.exists() || !path.is_dir() {
            return Err(String::from("Wrong path"));
        }

        file_system.change_path(&path);
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