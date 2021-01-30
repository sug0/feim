use std::io::{self, Write, BufReader, BufWriter};

use feim::image::{
    Codec,
    Format,
    DecodeOptions,
    farbfeld::Farbfeld,
};
use feim::serialize::try_format;
use feim::buffer::RawPixBuf;
use feim::color::Nrgba64;

struct Dummy;

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut stdin_reader = BufReader::new(stdin_lock);

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let formats: [&dyn Format; 2] = [
        &Farbfeld,
        &Dummy,
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
        Ok(_) => unreachable!(),
        Err(e) => Err(e),
    }
}

impl Format for Dummy {
    fn id(&self) -> &'static str { "" }

    fn magic(&self) -> &'static [u8] { b"xxxxxxxx" }
}
