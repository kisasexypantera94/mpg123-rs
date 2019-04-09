# mpg123-rs
[![Latest Version](https://img.shields.io/crates/v/mpg123.svg)](https://crates.io/crates/mpg123)


## About
An opinionated wrapper for mpg123-sys.

## Usage
```rust
let decoder = Decoder::new("assets/a-Ha - Take On Me.mp3", None).unwrap();
println!("{:?}", decoder.format());
let mut samples = Vec::new();

loop {
    let mut buf = vec![0; 2048];
    match decoder.read(&mut buf) {
        Ok(()) => {
            for x in buf.into_iter() {
                samples.push(x);
            }
        }
        Err(Error::EOF) => break,
        Err(e) => panic!(e),
    }
}
```

Other examples can be found [here](https://github.com/kisasexypantera94/mpg123-rs/blob/master/tests/integration_test.rs).