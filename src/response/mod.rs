//! An augmentation of the rust-http Response struct.

use std::io::{IoResult, File};
use std::io::util::copy;
use std::path::BytesContainer;

use http::status::Status;

pub use Response = http::server::response::ResponseWriter;

use self::mimes::get_generated_content_type;

mod mimes;

/// Adds common serving methods to Response.
pub trait Serve: Writer {
    /// Serve the file located at `path`.
    ///
    /// This is usually a terminal process, and `Middleware` may want to
    /// call `Unwind` after a file is served. If the status should be
    /// anything other than `200`, the `Middleware` must set it, including in
    /// the case of an `Err`.
    ///
    /// `serve_file` will err out if the file does not exist, the process
    /// does not have correct permissions, or it has other issues in reading
    /// from the file. Middleware should handle this gracefully.
    fn serve_file(&mut self, &Path) -> IoResult<()>;

    /// Write the `Status` and data to the `Response`.
    ///
    /// `serve` will forward write errors to its caller.
    fn serve<S: BytesContainer>(&mut self, status: Status, body: S) -> IoResult<()>;
}

impl<'a> Serve for Response<'a> {
    fn serve_file(&mut self, path: &Path) -> IoResult<()> {
        let mut file = try!(File::open(path));
        self.headers.content_type = path.extension_str().and_then(get_generated_content_type);
        copy(&mut file, self)
    }

    fn serve<S: BytesContainer>(&mut self, status: Status, body: S) -> IoResult<()> {
        self.status = status;
        Ok(try!(self.write(body.container_as_bytes())))
    }
}

#[test]
fn matches_content_type () {
    let path = &Path::new("test.txt");
    let content_type = path.extension_str().and_then(get_generated_content_type).unwrap();

    assert_eq!(content_type.type_.as_slice(), "text");
    assert_eq!(content_type.subtype.as_slice(), "plain");
}
