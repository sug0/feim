use std::io::{self, Write, BufReader, BufWriter};

use feim::image::{
    Format,
    jpeg::Jpeg,
    farbfeld::Farbfeld,
};
use feim::serialize::{
    Decode,
    try_format,
    GenericDecodeOptions,
};
use feim::buffer::RawPixBuf;
use feim::color::{Nrgba64, BigEndian};

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

    match try_format(&mut stdin_reader, &formats[..]) {
        Ok(0) => {
            let opts = GenericDecodeOptions {
                check_header: false,
            };
            let image: RawPixBuf<Nrgba64<BigEndian>> = Farbfeld::decode(stdin_reader, opts)?;
            write!(&mut stdout_writer, "{:#?}", image).unwrap_or(());
            Ok(())
        },
        Ok(1) => {
            let image = Jpeg::decode(stdin_reader, ())?;
            write!(&mut stdout_writer, "{:#?}", image).unwrap_or(());
            Ok(())
        },
        Ok(_) => unreachable!(),
        Err(e) => Err(e),
    }
}
