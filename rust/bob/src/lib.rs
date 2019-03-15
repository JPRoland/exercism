pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    match trimmed {
        trimmed if trimmed.is_empty() => "Fine. Be that way!",
        trimmed if is_question(trimmed) && is_yell(trimmed) => "Calm down, I know what I'm doing!",
        trimmed if is_question(trimmed) => "Sure.",
        trimmed if is_yell(trimmed) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}

fn is_question(msg: &str) -> bool {
    msg.ends_with('?')
}

fn is_yell(msg: &str) -> bool {
    let mut contains_letters = false;
    for c in msg.chars() {
        if c.is_alphabetic() {
            contains_letters = true;
            if !c.is_uppercase() {
                return false;
            }
        }
    }

    contains_letters
}
