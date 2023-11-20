#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::rngs::ThreadRng;

use crate::funk::*;
use std::f32::consts::TAU as tau;
pub fn s(t:f32,d:f32,i:f32,r:&mut ThreadRng,b:&mut buffer)->f32{
    let f = 440.0f32;
    let a = 0.2f32;
    (((f*tau*t).sin())*0.2+((f*2.0*tau*t*(1.0-t/6.0)).sin())*0.2)*0.2
}
