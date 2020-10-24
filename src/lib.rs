use std::ffi::CString;
use std::io::{BufReader, Cursor, Read};

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    let asset_manager = ndk_glue::native_activity().asset_manager();
    let asset = asset_manager
        .open(&CString::new("Windless Slopes.mp3").unwrap())
        .unwrap();

    let mut data = Vec::with_capacity(asset.get_length());
    BufReader::new(asset).read_to_end(&mut data).unwrap();

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    stream_handle
        .play_once(Cursor::new(data))
        .unwrap()
        .sleep_until_end();
}
