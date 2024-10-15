fn normalize_spaces(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn main() {
    let orig_title = "Заголовок:\n       Подзаголовок ыа я ";

    println!("{}", normalize_spaces(&orig_title))
}
