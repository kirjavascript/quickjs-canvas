pub fn to_canvas(text: &str) -> String {
    text.replace(char::is_whitespace, " ")
}
