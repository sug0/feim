//! [`feim`](https://sr.ht/~sugo/feim) is a crate designed with some clues from
//! [libgoimg](https://github.com/sug0/libgoimg), which in turn was heavily influeced
//! by the API of [Go's image package](https://pkg.go.dev/image). One of the goals
//! of `feim` was to increase the flexibility users had to work with images, namely
//! rolling their own custom serialization routines, color formats and image formats.
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
//! fn draw_image(buf: &mut RawPixBuf<Rgb>) {
//!     const WHITE: Rgb = Rgb { r: 255, g: 255, b: 255 };
//!
//!     for y in 0..DIM {
//!         for x in 0..DIM {
//!             buf.pixel_set(x, y, WHITE);
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
//!     // NOTE: you should be using [`feim::image::BuiltInFormat`]
//!     // as the tag type instead of integers. a readily available
//!     // iterator with all built-in formats is available from
//!     // [`feim::image::built_in_formats_iter`].
//!     let formats: [&dyn Format; 2] = [
//!         (0, &Farbfeld),
//!         (1, &Jpeg),
//!         // ...
//!     ];
//!
//!     match try_format(&mut stdin_reader, formats) {
//!         Ok(0) => {
//!             let opts = FarbfeldDecodeOptions {
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

pub mod buffer;
pub mod color;
pub mod image;
pub mod serialize;
pub mod specialized;
