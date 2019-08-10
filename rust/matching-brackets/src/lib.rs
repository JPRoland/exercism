use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    let brackets: HashMap<char, char> = [('(', ')'), ('{', '}'), ('[', ']')]
        .iter()
        .cloned()
        .collect();

    for c in string.chars() {
        if !brackets.contains_key(&c) && !brackets.values().any(|&x| x == c) {
            continue;
        }

        if brackets.contains_key(&c) {
            stack.push(c);
            continue;
        }

        match stack.pop() {
            Some(b) if brackets[&b] == c => continue,
            _ => return false,
        }
    }

    stack.is_empty()
}
