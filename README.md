# Basic http client example with Rust

I am learning Rust and making a simple HTTP request seemed too complicated. The examples provided were using advanced constructs such as Boxes, required tricks for creating a simple blocked request.

I wanted the equivalent of requests.get(x).text in python.

So, I hacked together a simple program showing the simplest http request example I could find. The example uses the reqwest library.

## Instructions

### Setting up a new project
> run the following command:
>    cargo new http-example

### Getting dependencies
> Put this in your Cargo.toml file:
> ```toml
[package]
name = "http-example"
version = "0.1.0"
authors = ["erkin"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
futures = "0.3"
reqwest = { version = "0.10"}
tokio = { version = "0.2", features = ["full"] }
```

### The actual code
> Put this in your src/main.rs
> ```rust
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let conn = reqwest::get("https://www.rust-lang.org");
    let content = block_on(conn).expect("Error Occured in establishing conneciton");
    let text = block_on(content.text()).expect("Error Occured in retrieving the text content");
    println!("Text: {}", text);
}
```