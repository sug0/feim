mod expression;

use std::io::{self, BufWriter};
use std::num::NonZeroUsize;

use clap::Parser;
use feim::buffer::RawPixBuf;
use feim::image::farbfeld::Farbfeld;
use feim::serialize::EncodeSpecialized;

const DIM: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(500) };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    expr: String,
    #[arg(short, long, default_value_t = expression::BitDepth::One)]
    depth: expression::BitDepth,
    #[arg(short, long, default_value_t = DIM)]
    width: NonZeroUsize,
    #[arg(short, long, default_value_t = DIM)]
    height: NonZeroUsize,
}

fn main() {
    let args = Args::parse();
    let expr = expression::compile(&args.expr).unwrap();

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(args.width.get(), args.height.get());
    expr.evaluate_over(&mut image, args.depth).unwrap();

    Farbfeld::encode_specialized(&mut stdout_writer, (), &image).unwrap()
}
