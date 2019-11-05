# FromString

Demo of Rust's [FromString Trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)

## Usage

First, clone the repo

```
git clone https://github.com/cgcardona/Rust_FromString.git
```

Next, change directories and build app and deps.

```
cd Rust_FromString/
cargo build
```

Now run the app

```
cargo run
```

## Things to note

- [&str to u32 w/ turbofish](./src/main.rs#L32)
- [&str to f32 w/ turbofish](./src/main.rs#L36)
- [&str to String w/ turbofish](./src/main.rs#L40)
- [&str to SocketAddr from TcpStream::connect](./src/main.rs#L44)
- [&str to Square's FromString implementation](./src/main.rs#L49)
