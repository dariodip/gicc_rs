use inquire::{
    error::InquireResult, max_length, required, validator::Validation, Confirm, Editor, Select,
    Text,
};

fn main() -> InquireResult<()> {
    let commit_type = Select::new("Type:", get_types()).prompt()?;
    let mut commit_message: String = commit_type.to_string();

    // prompt scope (if any)
    let scope = Text::new("Scope:")
        .with_validator(max_length!(10))
        .prompt()?;
    commit_message = if scope.is_empty() {
        commit_message
    } else {
        format!("{}({}):", commit_message, scope)
    };

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

    let body = Editor::new("Body:").prompt()?;

    print!("{}{}\n{}", commit_message, description, body);

    Ok(())
}

fn get_types() -> Vec<&'static str> {
    vec![
        "build", "ci", "docs", "feat", "fix", "perf", "refactor", "style", "test",
    ]
}
