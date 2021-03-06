extern crate flate2;

use std::io::prelude::*;
use std::io;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::bufread::ZlibDecoder;

// Compress a sample string and print it after transformation.
fn main() {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::Default);
    e.write(b"Hello World").unwrap();
    let bytes = e.finish().unwrap();
    println!("{}", decode_bufreader(bytes).unwrap());
}

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead
fn decode_bufreader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
