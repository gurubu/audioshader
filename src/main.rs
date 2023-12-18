#![allow(non_camel_case_types,unused_imports)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
use bqn::bqntosound;
use rand::SeedableRng;
use rodio::{Sink, self, buffer::SamplesBuffer, OutputStream};
use pest::*;
use pest_derive::*;
pub mod dsp;
pub mod funk;
pub mod bqn;
use crate::{dsp::*, funk::*};
const sff:f32=44100.0;
const fsf:f32 = 1.0/sff;
const bit:f32=16.0;
fn main() {
    let mut rng = rand::thread_rng();
    let mut rngsd = rand_chacha::ChaCha8Rng::seed_from_u64(0);
    let (_str,strh) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&strh).unwrap();
    // let mut w:Vec<f32>=vec![];
    // macro_rules!dds{($d:expr)=>{($d*bit*sff)as usize};}
    // let dur:usize = dds!(0.5);
    // let mut bv:Vec<buffer>=vec![];
    // bv.push(buffer::new(dur,5000));
    // bv.push(buffer::new(dur,100));
    // for x in 0..dur{
    //     let xf = x as f32;
    //     w.push(s(xf/sff,dur as f32,x as f32,&mut rng,&mut bv));
    // }
    let b:Vec<f32> = bqntosound();
    let r = a();
    let s = SamplesBuffer::new(1,sff as u32,b);
    sink.append(s);
    sink.sleep_until_end();
}
