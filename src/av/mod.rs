use std::fs::File;

pub struct Stream {
    index: i8,
    media_type: String,
    params: StreamParams,
    codec_params: Vec<String>
}

pub struct StreamParams {
    codec: i32,
    fps: f64,
    start: f64,
    dur: f64
}

pub struct Operation {
    from_stream: Stream,
    to_stream: Stream,
}
