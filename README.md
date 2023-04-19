# kizuna

[![CI-badge]](ci)

[CI-badge]: https://github.com/Neo-Ciber94/kizuna/actions/workflows/ci.yml/badge.svg
[ci]: https://github.com/Neo-Ciber94/kizuna/actions/workflows/ci.yml

🔍 A simple service locator for Rust.

This library provides a simple service locator for Rust programs. It allows for easy insertion and retrieval of values by type, and supports both single instance values and values created by factory functions.

## Add to your project

```bash
cargo add kizuna
```

or in your `Cargo.toml`

```toml
kizuna = "0.1.0"
```

## Usage

```rust
use kizuna::Locator;

#[derive(Debug, Clone, PartialEq)]
struct Name(&'static str);

#[derive(Debug)]
struct Person(Name);

fn greet(person: Person) {
    println!("Hello {:?}", person.0);
}

fn main() {
    let mut locator = Locator::new();
    
    // Register the dependencies
    locator.insert(Name("Athena"));
    locator.insert_with(|locator| Person(locator.get::<Name>().unwrap()));

    let person = locator.get::<Person>().unwrap();
    assert_eq!(person.0, Name("Athena"));

    // You can call a function injecting the dependencies
    locator.invoke(greet).unwrap();
}
```

## Support for `async/await`

```rust
use kizuna::Locator;

async fn hello(greet: String) -> usize {
    println!("{greet}");
    greet.len()
}

#[tokio::main]
async fn main() {
    let mut locator = Locator::new();
    locator.insert(String::from("hello world"));

    let result = locator.invoke_async(hello).await.unwrap();
    assert_eq!(result, 11);
}
```

## Test

Run tests with `cargo test --lib`

## License

This project is licensed under the MIT License
