const WIDTH: usize = 1;

pub fn wide(s: &str) -> String {
    let mut output = String::new();

    for c in s.as_bytes() {
        output.push_str(
            String::from(
                format!("{}{}", *c as char, " ".repeat(WIDTH))
            ).as_str()
        )
    }

    return output;
}
