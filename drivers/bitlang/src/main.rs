mod expression;

use std::collections::HashMap;
use std::io::{self, BufWriter};
use std::num::NonZeroUsize;

use clap::Parser;
use feim::buffer::RawPixBuf;
use feim::image::farbfeld::Farbfeld;
use feim::serialize::EncodeSpecialized;
use serde::Deserialize;

const DIM: NonZeroUsize = unsafe { NonZeroUsize::new_unchecked(500) };

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CmdLineArgs {
    /// Expression used to generate the image; consider an expression
    /// alias, if we are parsing a TOML file of expressions
    #[arg(short, long)]
    expr: String,
    /// Bit-depth of the resulting image
    #[arg(short, long, default_value_t = expression::BitDepth::One)]
    depth: expression::BitDepth,
    /// Width of the resulting image
    #[arg(short = 'W', long, default_value_t = DIM)]
    width: NonZeroUsize,
    /// Height of the resulting image
    #[arg(short = 'H', long, default_value_t = DIM)]
    height: NonZeroUsize,
    /// Optional TOML file with expression definitions
    #[arg(long)]
    toml: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ExpressionArgs {
    generate: String,
    depth: expression::BitDepth,
    width: NonZeroUsize,
    height: NonZeroUsize,
}

#[derive(Deserialize, Debug)]
struct ExpressionsToml {
    expressions: HashMap<String, ExpressionArgs>,
}

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = get_expression_args()?;
    let expr = expression::compile(&args.generate)?;

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(args.width.get(), args.height.get());
    expr.evaluate_over(&mut image, args.depth)?;

    Farbfeld::encode_specialized(&mut stdout_writer, (), &image)?;
    Ok(())
}

fn get_expression_args() -> Result<ExpressionArgs> {
    let CmdLineArgs {
        expr,
        depth,
        width,
        height,
        toml,
    } = CmdLineArgs::parse();
    Ok(if let Some(path) = toml {
        let data = std::fs::read(path)?;
        let mut toml: ExpressionsToml = toml::from_slice(&data)?;
        toml.expressions
            .remove(&expr)
            .ok_or_else(|| format!("No expression alias found: {expr}"))?
    } else {
        ExpressionArgs {
            generate: expr,
            depth,
            width,
            height,
        }
    })
}
