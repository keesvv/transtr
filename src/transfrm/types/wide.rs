const WIDTH: usize = 1;

pub fn wide(s: &str) -> String {
    let mut output = String::new();

    for c in s.chars() {
        output.push_str(
            String::from(
                format!("{}{}", c, " ".repeat(WIDTH))
            ).as_str()
        )
    }

    return output;
}
