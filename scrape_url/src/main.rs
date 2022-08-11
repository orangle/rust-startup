use std::fs;

fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "/tmp/rust.md";

    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Convertin html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output, md.as_bytes()).unwrap();
    println!("Converted md hs been save in {}.", output);
}
