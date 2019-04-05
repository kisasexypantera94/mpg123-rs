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
pub struct Format {
    rate: i64,
    channels: i32,
    encoding: i32,
}

impl Default for Format {
    fn default() -> Format {
        Format {
            rate: 0,
            channels: 0,
            encoding: 0,
        }
    }
}

pub struct Decoder {
    mh: *mut mpg123::mpg123_handle,
    format: Format,
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

            let mut decoder = Decoder {
                mh,
                format: Format::default(),
            };

            decoder.open(filename)?;
            decoder.get_format()?;

            Ok(decoder)
        }
    }

    fn open(&self, filename: &str) -> Result<(), Box<StdError>> {
        let fname = ffi::CString::new(filename)?;

        unsafe {
            if mpg123::mpg123_open(self.mh, fname.as_ptr()) != mpg123::MPG123_OK as c_int {
                return Err(Box::from(format!("failed to open `{}`", filename)));
            }
        }

        Ok(())
    }

    fn get_format(&mut self) -> Result<(), Box<StdError>> {
        unsafe {
            if mpg123::mpg123_getformat(
                self.mh,
                &mut self.format.rate,
                &mut self.format.channels,
                &mut self.format.encoding,
            ) != mpg123::MPG123_OK as c_int
            {
                return Err(Box::from("failed to get format"));
            }
        }

        Ok(())
    }

    pub fn format(&self) -> &Format {
        &self.format
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
        if !self.mh.is_null() {
            self.close().unwrap_or_else(|e| println!("{}", e));
            self.delete();
        }
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
