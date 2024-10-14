use dom_readability::Readability;

fn main() {
    let html = include_str!("../test-pages/replace-brs/source.html");
    let mut readability = Readability::from(html);

    println!("title: {}", &readability.get_title());

    readability.prepare();

    let contents = readability.doc.html();
    std::fs::write("result.html", contents.as_bytes()).unwrap();
}
