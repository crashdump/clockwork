use crate::task::Command;

pub(crate) struct Email {
    pub(crate) args: String,
}

impl Command for Email {
    fn run(&self) -> Result<String, String> {
        Ok(format!("{}", self.args))
    }

    fn name(&self) -> &'static str {
        "email"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs() {
        let echo = Email {
            args: "foo".to_string(),
        };
        assert_eq!(echo.name(), "echo");
        assert_eq!(echo.run().unwrap(), "foo");
    }
}