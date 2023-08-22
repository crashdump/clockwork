use crate::task::Command;

pub struct Echo {
    pub args: String,
}

impl Command for Echo  {
    fn name(&self) -> &'static str { "echo" }
    fn run(&self) -> Result<String, String> {
        Ok("sucesfully ran echo".to_string())
    }
}