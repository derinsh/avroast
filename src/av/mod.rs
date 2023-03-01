use std::fs::File;

pub struct Stream {
    index: i8,
    src_c: String,
    dst_c: String,
    file: Box<File>,
    dur: i64,
    codec_info: Vec<String>
}
