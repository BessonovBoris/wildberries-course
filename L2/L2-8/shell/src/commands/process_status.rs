use clap::Parser;
use crate::{Command, FileSystem};

#[derive(Parser)]
pub struct ProcessStatus {
    #[clap(default_value = "0")]
    pid: u32,

    #[clap(skip)]
    next: Option<Box<dyn Command>>,
}

impl ProcessStatus {
    pub fn from_input(path: &Vec<&str>) -> Self {
        ProcessStatus::parse_from(path.iter())
    }
}

impl Command for ProcessStatus {
    fn execute(&mut self, file_system: &mut FileSystem) -> Result<Vec<String>, String> {
        let process = std::process::Command::new("powershell")
            .arg("ps")
            .output()
            .unwrap();

        let process = String::from_utf8_lossy(&process.stdout).to_string();
        let process = process.lines().map(|x| x.to_string()).collect::<Vec<String>>();

        if let Some(next) = self.next.as_mut() {
            next.execute(file_system)
        } else {
            Ok(process)
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