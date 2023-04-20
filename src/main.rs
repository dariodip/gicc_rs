use inquire::{
    error::InquireResult, max_length, required, validator::Validation, Confirm, Editor, Select,
    Text,
};
use std::io::{self, Write};
use std::{
    io::Error,
    process::{Command, Output},
};

fn main() -> InquireResult<()> {
    loop {
        let commit_message: String = user_prompt()?;

        println!("Your commit message looks like this:");
        println!("{}", commit_message);
        let confirm_ok = Confirm::new("Do you want to proceed?")
            .with_default(true)
            .prompt()?;

        if confirm_ok {
            match commit(commit_message) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
            break;
        }
        println!("Ok! Let's redo");
    }

    Ok(())
}

fn user_prompt() -> Result<String, inquire::InquireError> {
    // get commit type
    let commit_type = Select::new("Type:", get_types()).prompt()?;
    let mut commit_message: String = commit_type.to_string();

    // get scope
    let scope = Text::new("Scope:")
        .with_validator(max_length!(10))
        .prompt()?;
    commit_message = if scope.is_empty() {
        commit_message
    } else {
        format!("{}({})", commit_message, scope)
    };

    // get description
    let cm_length = commit_message.len();
    let message_length_validator = move |msg: &str| {
        if cm_length + msg.len() > 50 {
            Ok(Validation::Invalid(
                "Message should not be more than 50 characters-----".into(),
            ))
        } else {
            Ok(Validation::Valid)
        }
    };
    let description = Text::new("Description:")
        .with_validator(message_length_validator)
        .with_validator(required!())
        .prompt()?;

    // get body
    let body = Editor::new("Body:").prompt()?;
    commit_message = if body.is_empty() {
        commit_message
    } else {
        format!("{}: {}\n\n{}", commit_message, description, body)
    };

    Ok(commit_message)
}

fn get_types() -> Vec<&'static str> {
    vec![
        "build", "ci", "docs", "feat", "fix", "perf", "refactor", "style", "test",
    ]
}

fn commit(message: String) -> Result<(), String> {
    match do_commit(message) {
        Ok(output) => {
            if output.status.success() {
                io::stdout().write_all(&output.stdout).unwrap();
                Ok(())
            } else {
                io::stderr().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
                Err(format!("Exited with status {}", output.status))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

fn do_commit(message: String) -> Result<Output, Error> {
    Command::new("git")
        .args(["commit", "-m", &message])
        .output()
}
