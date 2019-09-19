//! An example of generating constant valued noise

extern crate noise;

use noise::utils::*;
use noise::Checkerboard;

fn main() {
    let checker = Checkerboard::new();

    PlaneMapBuilder::new(&checker)
        .build()
        .write_to_file("checkerboard.png");
}
