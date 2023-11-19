#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::rngs::ThreadRng;

use crate::funk::*;
use std::f32::consts::TAU as tau;
pub fn s(t:f32,d:f32,i:f32,r:&mut ThreadRng)->f32{
    //let f = 440.0f32;
    //limit((f*tau*t).sin(),0.4)*0.6
    if i<sectosample(0.01){
        noise(r)*0.4
    }
    else{
        0.0
    }
}
