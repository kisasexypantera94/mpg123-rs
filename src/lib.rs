mod error;

pub use error::Error;
use libc::c_int;
use mpg123_sys as mpg123;
use std::error::Error as StdError;
use std::ffi;
use std::ops::Drop;
use std::ptr;
use std::sync::Once;

static INIT: Once = Once::new();

#[derive(Debug)]
struct Format {
    rate: i64,
    channels: i32,
    encoding: i32,
}

struct Decoder {
    mh: *mut mpg123::mpg123_handle,
    pub format: Format,
}

impl Decoder {
    pub fn new(filename: &str) -> Result<Decoder, Box<StdError>> {
        unsafe {
            init()?;

            let mut res = 0;
            let mh = mpg123::mpg123_new(ptr::null(), &mut res);
            if res != mpg123::MPG123_OK as c_int || mh.is_null() {
                return Err(Box::from("failed to instantiate mpg123"));
            }

            open(mh, filename)?;
            let format = get_format(mh)?;
            let decoder = Decoder { mh, format };
            Ok(decoder)
        }
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<(), Error> {
        unsafe {
            let mut done = 0;
            let res = mpg123::mpg123_read(self.mh, buf.as_mut_ptr(), buf.len(), &mut done);
            if res != mpg123::MPG123_OK as c_int && res != mpg123::MPG123_DONE as c_int {
                return Err(Error::BadInput);
            }
            if res == mpg123::MPG123_DONE as c_int {
                return Err(Error::EOF);
            }
        }

        Ok(())
    }

    fn close(&self) -> Result<(), Box<StdError>> {
        unsafe {
            if mpg123::mpg123_close(self.mh) != mpg123::MPG123_OK as c_int {
                return Err(Box::from("failed to close"));
            }
        }

        Ok(())
    }

    fn delete(&self) {
        unsafe {
            mpg123::mpg123_delete(self.mh);
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        self.close().unwrap();
        self.delete();
    }
}

fn init() -> Result<(), Box<StdError>> {
    let mut result = Ok(());

    INIT.call_once(|| unsafe {
        if mpg123::mpg123_init() != mpg123::MPG123_OK as c_int {
            result = Err(Box::from("Failed to initialize mpg123"));
        }
    });

    result
}

fn open(mh: *mut mpg123::mpg123_handle, filename: &str) -> Result<(), Box<StdError>> {
    let fname = ffi::CString::new(filename)?;

    unsafe {
        if mpg123::mpg123_open(mh, fname.as_ptr()) != mpg123::MPG123_OK as c_int {
            return Err(Box::from(format!("failed to open `{}`", filename)));
        }
    }

    Ok(())
}

fn get_format(mh: *mut mpg123::mpg123_handle) -> Result<Format, Box<StdError>> {
    unsafe {
        let mut rate = 0;
        let mut channels = 0;
        let mut encoding = 0;
        if mpg123::mpg123_getformat(mh, &mut rate, &mut channels, &mut encoding)
            != mpg123::MPG123_OK as c_int
        {
            println!("{},{},{}", rate, channels, encoding);
            return Err(Box::from("failed to get format"));
        }

        Ok(Format {
            rate,
            channels,
            encoding,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Decoder, Error};

    #[test]
    fn test_decode() {
        let decoder = Decoder::new("assets/a-Ha - Take On Me.mp3").unwrap();
        println!("{:?}", decoder.format);
        let mut samples = Vec::new();

        loop {
            let mut buf = vec![0; 1024];
            match decoder.read(&mut buf) {
                Ok(()) => {
                    for x in buf.into_iter() {
                        samples.push(x);
                    }
                }
                Err(e) => match e {
                    Error::EOF => break,
                    other => panic!(other),
                },
            }
        }
    }
}
