use std::path::PathBuf;
use clap::Parser;
use crate::Command;
use crate::file_system::FileSystem;

#[derive(Parser)]
pub struct List {
    #[clap(default_value = ".\\")]
    path: PathBuf,

    #[clap(skip)]
    next: Option<Box<dyn Command>>,
}

impl List {
    pub fn from_input(path: &Vec<&str>) -> List {
        List::parse_from(path.iter())
    }
}

impl Command for List {
    fn execute(&mut self, file_system: &mut FileSystem) -> Result<Vec<String>, String> {
        let path = file_system.path().join(&self.path);

        if !path.exists() || !path.is_dir() {
            return Err(format!("{:?} does not exist", path));
        }

        let files = std::fs::read_dir(path)
            .unwrap()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|x| x.unwrap().file_name().into_string().unwrap())
            .collect::<Vec<_>>();

        if let Some(next) = self.next.as_mut() {
            next.set_arg(files);
            next.execute(file_system)
        } else {
            Ok(files)
        }
    }

    fn set_arg(&mut self, _arg: Vec<String>) {
    }

    fn add_next(&mut self, command: Box<dyn Command>) {
        if let Some(next) = self.next.as_mut() {
            next.add_next(command);
            return;
        }

        self.next = Some(command);
    }
}