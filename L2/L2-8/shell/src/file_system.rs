use std::path::PathBuf;
use crate::command::Command;

pub struct FileSystem {
    path: PathBuf,
}

impl FileSystem {
    pub fn from_input(path: &str) -> FileSystem {
        let path = PathBuf::from(path);
        let absolute_path = path.canonicalize().unwrap();

        FileSystem { path: absolute_path }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub(crate) fn change_path(&mut self, path: &PathBuf) {
        self.path.push(path);

        if !self.path.exists() {
            panic!("Path does not exist: {:?}", self.path);
        }
    }

    pub fn execute(&mut self, mut command: Box<dyn Command>) -> Result<(), String> {
        let res = command.execute(self);
        match res {
            Ok(res) => {
                for e in res {
                    println!("{}", e);
                }
                println!();
            }
            Err(e) => return Err(e.to_string()),
        }

        Ok(())
    }
}