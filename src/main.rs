#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use rodio::{Sink, self, buffer::SamplesBuffer, OutputStream};
pub mod dsp;
pub mod funk;
use crate::dsp::*;
const sff:f32=44100.0;
fn main() {
    let mut rng = rand::thread_rng();
    let (_str,strh) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&strh).unwrap();
    let mut w:Vec<f32>=vec![];
    macro_rules!dds{($d:expr)=>{($d*16.0*44100.0)as usize};}
    let dur:usize = dds!(0.1);
    for x in 0..dur{
        let xf = x as f32;
        w.push(s(xf/sff,dur as f32,x as f32,&mut rng));
    } 
    let s = SamplesBuffer::new(1,44100,w);
    sink.append(s);
    sink.sleep_until_end();
}
