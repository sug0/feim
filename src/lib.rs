//! [`feim`](https://sr.ht/~sugo/feim) is a crate designed with some clues from
//! [libgoimg](https://github.com/sug0/libgoimg), which in turn was heavily influeced
//! by the API of [Go's image package](https://pkg.go.dev/image). One of the goals
//! of `feim` was to increase the flexibility users had to work with images, namely
//! rolling their own custom serialization routines, color formats and image formats.
//! I believe this makes `feim` the best image processing library available in the Rust
//! ecosystem, at the moment.
//!
//! ## Encoding
//!
//! Here is a simple white background being saved to a file `out.png`:
//!
//! ```rust
//! const DIM: usize = 250;
//!
//! fn main() -> io::Result<()> {
//!     let output = File::create("out.png")?;
//!     let output = BufWriter::new(output);
//!
//!     let mut image = RawPixBuf::new(DIM, DIM);
//!     draw_image(image.as_typed_mut());
//!
//!     let opts = Default::default();
//!     Png::encode(output, opts, &image)
//! }
//!
//! fn draw_image(buf: &mut [Rgb]) {
//!     const WHITE: Rgb = Rgb { r: 255, g: 255, b: 255 };
//!
//!     for y in 0..DIM {
//!         for x in 0..DIM {
//!             buf[y*DIM + x] = WHITE;
//!         }
//!     }
//! }
//! ```
//!
//! ## Decoding
//!
//! Decoding images with `feim` is as simples as:
//!
//! ```rust
//! fn main() -> io::Result<()> {
//!     let stdin = io::stdin();
//!     let stdin_lock = stdin.lock();
//!     let mut stdin_reader = BufReader::new(stdin_lock);
//!
//!     let stdout = io::stdout();
//!     let stdout_lock = stdout.lock();
//!     let mut stdout_writer = BufWriter::new(stdout_lock);
//!
//!     let formats: [&dyn Format; 2] = [
//!         &Farbfeld,
//!         &Jpeg,
//!         // ...
//!     ];
//!
//!     match try_format(&mut stdin_reader, &formats[..]) {
//!         Ok(0) => {
//!             let opts = GenericDecodeOptions {
//!                 check_header: false,
//!             };
//!             let image: RawPixBuf<Nrgba64Be> = Farbfeld::decode(stdin_reader, opts)?;
//!             let _ = write!(&mut stdout_writer, "{:#?}", image);
//!             Ok(())
//!         },
//!         Ok(1) => {
//!             let image = Jpeg::decode(stdin_reader, ())?;
//!             let _ = write!(&mut stdout_writer, "{:#?}", image);
//!             Ok(())
//!         },
//!         Ok(_) => unreachable!(),
//!         Err(e) => Err(e),
//!     }
//! }
//! ```

#![feature(specialization)]
#![allow(incomplete_features)]

pub mod buffer;
pub mod color;
pub mod image;
pub mod serialize;
