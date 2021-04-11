use std::io::{self, Write, BufReader, BufWriter};

use feim::image::{
    Format,
    jpeg::Jpeg,
    farbfeld::Farbfeld,
};
use feim::serialize::{
    Decode,
    try_format,
    DecodeOptions,
};
use feim::buffer::RawPixBuf;
use feim::color::Nrgba64;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let formats: [&dyn Format; 2] = [
        &Farbfeld,
        &Jpeg,
        // ...
    ];
    let opts = DecodeOptions {
        check_header: false,
    };

    match try_format(&mut stdin_reader, &formats[..]) {
        Ok(0) => {
            let image: RawPixBuf<Nrgba64> = Farbfeld::decode(stdin_reader, opts)?;
            write!(&mut stdout_writer, "{:#?}", image).unwrap_or(());
            Ok(())
        },
        Ok(1) => {
            let image = Jpeg::decode(stdin_reader, opts)?;
            write!(&mut stdout_writer, "{:#?}", image).unwrap_or(());
            Ok(())
        },
        Ok(_) => unreachable!(),
        Err(e) => Err(e),
    }
}
