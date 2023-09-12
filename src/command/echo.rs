use crate::task::Command;

pub(crate) struct Echo {
    pub(crate) args: String,
}

impl Command for Echo {
    fn run(&self) -> Result<String, String> {
        Ok(format!("echo: {}", self.args))
    }

    fn name(&self) -> &'static str {
        "echo"
    }
}
