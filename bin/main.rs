use rookie;

fn main() {
    let domains = vec!["facebook.com"];
    let cookies = rookie::chrome_v2(Some(domains)).unwrap();
    println!("{:?}", cookies);
}
