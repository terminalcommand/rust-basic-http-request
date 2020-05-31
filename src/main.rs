use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let conn = reqwest::get("https://www.rust-lang.org");
    let content = block_on(conn).expect("Error Occured in establishing conneciton");
    let text = block_on(content.text()).expect("Error Occured in retrieving the text content");
    println!("Text: {}", text);
}
