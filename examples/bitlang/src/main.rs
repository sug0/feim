mod expression;

use std::env;
use std::io::{self, BufWriter};

use feim::buffer::RawPixBuf;
use feim::image::farbfeld::Farbfeld;
use feim::serialize::EncodeSpecialized;

const DIM: usize = 1000;

fn main() {
    let expr = expression::compile(&env::args().skip(1).by_ref().next().unwrap()).unwrap();

    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut stdout_writer = BufWriter::new(stdout_lock);

    let mut image = RawPixBuf::new(DIM, DIM);
    expr.evaluate_over(&mut image, expression::BitDepth::One)
        .unwrap();

    Farbfeld::encode_specialized(&mut stdout_writer, (), &image).unwrap()
}
