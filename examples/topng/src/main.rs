fn main() {}
// ```ignore
// use std::io::{self, BufReader, BufWriter};
//
// use feim::buffer::{RawPixBuf, AsTypedMut};
// use feim::color::{Nrgba64Be, Nrgba64Ne};
// use feim::image::{farbfeld::Farbfeld, png::Png};
// use feim::serialize::{Decode, Encode, EncodeSpecialized, GenericDecodeOptions};
//
// fn main() -> io::Result<()> {
//     let stdin = io::stdin();
//     let stdin_lock = stdin.lock();
//     let mut stdin_reader = BufReader::new(stdin_lock);
//
//     let stdout = io::stdout();
//     let stdout_lock = stdout.lock();
//     let stdout_writer = BufWriter::new(stdout_lock);
//
//     let opts = GenericDecodeOptions {
//         check_header: false,
//     };
//     let image = Farbfeld::decode(stdin_reader, opts)?;
//     Png::encode(stdout_writer, (), &image)
// }
//
// fn swap_bytes(mut image: RawPixBuf<Nrgba64Be>) -> RawPixBuf<Nrgba64Ne> {
//     let buf = image.as_typed_mut();
//
//     for pixel in buf.iter_mut() {
//         *pixel =
//     }
//
//     iamge.
// }
// ```
