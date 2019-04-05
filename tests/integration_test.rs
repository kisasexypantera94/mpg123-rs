extern crate mpg123;

use mpg123::*;

#[test]
fn test_decode() {
    let decoder = Decoder::new("assets/a-Ha - Take On Me.mp3").unwrap();
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
}