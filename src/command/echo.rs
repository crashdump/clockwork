use crate::task::Command;

pub(crate) struct Echo {
    pub(crate) args: String,
}

impl Command for Echo {
    fn run(&self) -> Result<String, String> {
        Ok(format!("{}", self.args))
    }

    fn name(&self) -> &'static str {
        "echo"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs() {
        let echo = Echo {
            args: "foo".to_string(),
        };
        assert_eq!(echo.name(), "echo");
        assert_eq!(echo.run().unwrap(), "foo");
    }
}