use std::io::{self, Read, Write, Cursor};
use refpack::{format::SimEA, CompressionOptions, compress};

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).expect("Failed to read stdin");

    let mut input_cursor = Cursor::new(&input);    // &[u8] + Seek
    let mut output_cursor = Cursor::new(Vec::new()); // Vec<u8> + Seek

    compress::<SimEA>(
        input.len(),
        &mut input_cursor,
        &mut output_cursor,
        CompressionOptions::Optimal
    ).expect("Compression failed");

    let compressed = output_cursor.into_inner();
    io::stdout().write_all(&compressed).expect("Failed to write to stdout");
}
