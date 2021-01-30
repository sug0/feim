use std::default::Default;
use std::io::{self, Write, BufReader, BufWriter};

use feim::image::{
    Codec,
    farbfeld::Farbfeld,
};
use feim::buffer::RawPixBuf;
use feim::color::Nrgba64;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let image: RawPixBuf<Nrgba64> = Farbfeld::decode(stdin_reader, Default::default())?;
    write!(&mut stdout_writer, "{:#?}", image).unwrap_or(());
    Ok(())
}
