#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::rngs::ThreadRng;

use crate::funk::*;
use std::f32::consts::TAU as tau;
pub fn s(t:f32,d:f32,i:f32,r:&mut ThreadRng,b:&mut buffer)->f32{
    let f = 440.0f32;
    let a = 0.2f32;
    //let mut o = 0.0;
    //let mut n = 0.0;
    //(((f*tau*t).sin())*0.2+((f*2.0*tau*t*(1.0-t/6.0)).sin())*0.2)*0.2
    //if i<sectosample(0.001){n = noise(r)*0.8;}
    //o = n + b.read();
    //b.write(o*0.9); 
    //o
    let mut o = 0.0;
    if i<d/8.0{o=(f*tau*t).sin()*a}
    o
}
