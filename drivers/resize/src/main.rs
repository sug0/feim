use std::io::{self, BufReader, BufWriter};

use clap::Parser;
use feim::buffer::{AsTyped, AsTypedMut, RawPixBuf};
use feim::color::{Nrgba64Be, Nrgba64Ne};
use feim::image::farbfeld::{Farbfeld, FarbfeldDecodeOptions};
use feim::image::Dimensions;
use feim::serialize::{Decode, Encode};
use resize_img::Pixel;
use resize_img::Type::Lanczos3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CmdLineArgs {
    /// The width of the output image
    #[arg(short = 'W', long)]
    width: usize,
    /// The height of the output image
    #[arg(short = 'H', long)]
    height: usize,
}

fn main() -> io::Result<()> {
    let args = CmdLineArgs::parse();

    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let stdout_writer = BufWriter::new(stdout_lock);

    let opts = FarbfeldDecodeOptions {
        check_header: false,
    };
    let image: RawPixBuf<Nrgba64Be> = Farbfeld::decode(stdin_reader, opts)?;
    Farbfeld::encode(stdout_writer, (), &resize(args, image))
}

fn resize(args: CmdLineArgs, orig: RawPixBuf<Nrgba64Be>) -> RawPixBuf<Nrgba64Ne> {
    let mut output = RawPixBuf::new(args.width, args.height);
    let mut resizer = resize_img::new(
        orig.width(),
        orig.height(),
        args.width,
        args.height,
        Pixel::RGBA16,
        Lanczos3,
    )
    .expect("Failed to create resizer");

    let src = unsafe { std::mem::transmute(orig.as_typed()) };
    let dst = unsafe { std::mem::transmute(output.as_typed_mut()) };
    resizer.resize(src, dst).expect("Resize failed");

    output
}
