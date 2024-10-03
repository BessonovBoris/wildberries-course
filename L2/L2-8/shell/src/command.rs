use crate::file_system::FileSystem;

pub trait Command {
    fn execute(&mut self, file_system: &mut FileSystem) -> Result<Vec<String>, String>;
    fn set_arg(&mut self, args: Vec<String>);
    fn add_next(&mut self, command: Box<dyn Command>);
}