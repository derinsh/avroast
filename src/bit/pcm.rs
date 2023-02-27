use std::io::BufReader;
use std::io::Read;

// https://ccrma.stanford.edu/courses/422-winter-2014/projects/WaveFormat/

struct WavParams {
    channels: i8,
    sample_rate: i32,
    bit_depth: i8,
    samples: i64
}

fn read_wav<R: Read>(reader : &BufReader<R>,
            data : &mut [i64],
            datasize : &u64,
            params : &WavParams)
{
    let (mut n_ch, mut s_rate, mut b_depth, mut samples) : (i8, i32, i8, i64) = (0,0,0,0);

    read_wav_header(&reader, &n_ch, &s_rate, &b_depth, &samples);

    ()
}

fn read_wav_header<R: Read>(reader : &BufReader<R>,
                   mut n_ch : &i8,
                   mut s_rate : &i32,
                   mut b_depth : &i8,
                   mut samples : &i64)
{
    ()
}
