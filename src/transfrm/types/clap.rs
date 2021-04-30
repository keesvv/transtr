pub fn clap(s: &str) -> String {
    let mut output = s.replace(" ", " 👏 ");
    output.push_str(" 👏");
    return output;
}
