pub fn capitalize(input_string: &str) -> String {
    let mut char_arr = input_string.chars();
    match char_arr.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + char_arr.as_str(),
    }
}

pub fn to_camel_case(input_string: &str)-> String {
    input_string
        .split("-")
        .enumerate()
        .map(|(i, word)| {
            if i < 1 {
                return word.to_string();
            }
            return capitalize(word);
        })
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests;