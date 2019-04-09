use mpg123::*;

#[test]
fn test_decode() {
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
}

use libc::c_long;
// Too lazy to add flags support to mpg123-sys
const MPG123_MONO_MIX: c_long = 0x4;
const MPG123_FORCE_FLOAT: c_long = 0x400;
#[test]
fn test_mono_decode() {
    let params = Some(MPG123_FORCE_FLOAT | MPG123_MONO_MIX);

    let decoder = Decoder::new("assets/a-Ha - Take On Me.mp3", params).unwrap();
    let format = decoder.format();
    assert_eq!(format.channels, 1);

    let mut mono_samples = Vec::new();

    loop {
        let mut buf = vec![0; 2048];
        match decoder.read(&mut buf) {
            Ok(()) => {
                for x in buf.into_iter() {
                    mono_samples.push(x);
                }
            }
            Err(Error::EOF) => break,
            Err(e) => panic!(e),
        }
    }
}
