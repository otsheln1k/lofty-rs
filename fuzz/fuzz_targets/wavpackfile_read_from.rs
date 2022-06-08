#![no_main]

use std::io::Cursor;

use libfuzzer_sys::fuzz_target;
use lofty::AudioFile;

fuzz_target!(|data: Vec<u8>| {
	let _ = lofty::wavpack::WavPackFile::read_from(&mut Cursor::new(data), false);
});
