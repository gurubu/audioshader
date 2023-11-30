#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use rodio::{Sink, self, buffer::SamplesBuffer, OutputStream};
pub mod dsp;
pub mod funk;
use crate::{dsp::*, funk::*};
const sff:f32=44100.0;
const bit:f32=16.0;
fn main() {
    let mut rng = rand::thread_rng();
    let (_str,strh) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&strh).unwrap();
    let mut w:Vec<f32>=vec![];
    let mut b = buffer::new(10000);
    macro_rules!dds{($d:expr)=>{($d*bit*sff)as usize};}
    let dur:usize = dds!(0.2);
    for x in 0..dur{
        let xf = x as f32;
        w.push(s(xf/sff,dur as f32,x as f32,&mut rng,&mut b));
    }
    let s = SamplesBuffer::new(1,sff as u32,w);
    sink.append(s);
    sink.sleep_until_end();
}
