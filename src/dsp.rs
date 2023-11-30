#![allow(unused_variables)]
#![allow(unused_imports)]
use rand::rngs::ThreadRng;

use crate::funk::*;
use std::f32::consts::TAU as tau;
pub fn s(t:f32,d:f32,i:f32,r:&mut ThreadRng,b:&mut buffer)->f32{
    let f:f32 = 
        220.0
        //+
        ////(tau*t).sin 
        //vibrato(tau*t,10.0,2.0)
        ;
    let a:f32 = 
        //(tau*t*2.0).sin().norm()*0.4
        0.2
        ;
    //let mut o = 0.0;
    //let mut n = 0.0;
    //(((f*tau*t).sin())*0.2+((f*2.0*tau*t*(1.0-t/6.0)).sin())*0.2)*0.2
    //if i<sectosample(0.001){n = noise(r)*0.8;}
    //o = n + b.read();
    //b.write(o*0.9); 
    //o
    let st = slope(i,d,0.0,1.0,0.0);
    //if i<sectosample(0.01){
    //  n = (f*tau*t).sin()*a;
    //}
    //o = n+b.read();
    //b.write(o*0.9);
    //b.varsize(st as usize);
    //(f*tau*t).sin()*((tau*t*0.5).tan().sin())
    let fm=(f*2.0*tau*t).sin();
    //(f+fm*tau*t).sin()*0.4
    //(((t*f)%2.0)-1.0)*0.2
    //+
    //(f*tau*t).sin()*0.4
    //((f*tau*t).sin()+(f*tau*(t/2.0)).sin())*0.5
    (440.0+((f*2.0*tau*t).sin()*0.5)*tau*t).sin()*0.1
}
