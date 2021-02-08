use person::Person;

#[actix_web::main]
async fn main() {
    let client = awc::Client::new();
    let p = Person{name: "Adam".to_owned(), height_cm: 181};
    let addr = std::env::var("SERVER_ADDR").unwrap();
    let url = format!("{}/{}/{}/index.html", addr, p.name, p.height_cm);
    println!("Requesting {}", url);
    let resp = client.get(url).send().await.unwrap();
    println!("{:?}", resp);
}