#![allow(unused_variables)]
use rand::{Rng, rngs::ThreadRng};
pub struct buffer{
    b:Vec<f32>,
    r:usize,
    w:usize,
}
impl buffer{
    pub fn new(size:usize)->Self{
        let mut b = buffer{b:vec![],r:0,w:0};
        for x in 0..size{
            b.b.push(0.0);
        }
        b
    }
    pub fn nom(&mut self,i:f32){
        self.b.push(i)
    }
}
pub fn norm(i:f32)->f32{
    (i+1.0)/2.0
}
pub fn clamp(i:f32,min:f32,max:f32)->f32{
    match i{
        i if i<min=>min,
        i if i>max=>max,
        _=>i,
    }
}
pub fn limit(i:f32,o:f32)->f32{
    match i{
        i if i>0.0=>o,
        i if i<=0.0=>-o,
        _=>i,
    }
}
pub fn lerp(v:Vec<f32>)->Vec<f32>{
    let mut vout:Vec<f32>=vec![];
    let i:usize=v.len();
    for x in 0..i{
        vout.push(v[x]);
        vout.push(v[x]+(v[x+1]-v[x]));
    }
    vout
}
pub fn noise(r:&mut ThreadRng)->f32{
    let x:f32 = r.gen();
    x
}
pub fn sampletosec(sample:f32)->f32{
    (sample/44100.0)/16.0
}
//pub fn delay(i:f32)
pub fn sectosample(sec:f32)->f32{
    sec*16.0*44100.0
}
