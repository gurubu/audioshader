#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
use rand::rngs::ThreadRng;
use crate::fsf;
use crate::funk::*;
use std::f32::consts::TAU as tau;
pub fn sp(t:f32,f:f32)->f32{
    (f*tau*t).sin()*(-3.0*t).exp()
}
pub fn fm(t:f32,fc:f32,fm:f32)->f32{
    (fc*tau*t+1.0*((fc*fm*tau*t).sin()*(-1.0*(t)).exp())).sin()*(-2.0*(t)).exp()
}
pub fn s(t:f32,d:f32,i:f32,r:&mut ThreadRng,b:&mut Vec<buffer>)->f32{
    let mut f:f32 = 
        440.0
        ;
    let a:f32 = 
        0.2
        ;
    let fs = stp(440.0,220.0,d);
    //let mut o = 0.0;
    //let mut n = 0.0;
    //(((f*tau*t).sin())*0.2+((f*2.0*tau*t*(1.0-t/6.0)).sin())*0.2)*0.2
    //if i<sectosample(0.001){n = noise(r)*0.8;}
    //o = n + b.read();
    //b.write(o*0.9); 
    //o
    //if i<sectosample(0.01){
    //  n = (f*tau*t).sin()*a;
    //}
    //o = n+b.read();
    //b.write(o*0.9);
    //b.varsize(st as usize);
    //(f*tau*t).sin()*((tau*t*0.5).tan().sin())
    //let imin = 0.0;
    //let imax = 9.0;
    //let im = t*(imax-imin)/d+imin;
    //(440.0+((f*2.0*tau*t).sin()*0.5)*tau*t).sin()*0.1
    //(tau*f*t+im*(tau*9.0*t).sin()).sin()*0.8
    //(f*tau*t*(1.0-t/)).sin()
    //let delsin = (((t*tau*0.2).sin()+1.0)/2.0)*1000.0+1.0;
    //let dels2n = (((t*tau*0.4).sin()+1.0)/2.0)* 500.0+40.0;
    let n = detns(i)*(-3.0*t).exp()*0.3;
    let d1= detns(del(i,600.0))*(-3.0*t).exp()*0.3;
    let d2= detns(del(i,1200.0))*(-3.0*t).exp()*0.3;
    let d3= detns(del(i,2400.0))*(-3.0*t).exp()*0.3;
    let d4= detns(del(i,4800.0))*(-3.0*t).exp()*0.3;
    n+d1+d2+d3+d4
    //b[0].varsize(delsin as usize);
    //b[0].write(n);
    //b[1].varsize(dels2n as usize);
    //b[1].write(n);
    //n
    //+
    //b[0].read()*0.3
    //+
    //b[1].read()*0.3
    
    //o
    //if tm(0.0,0.05,i){
    //    f=220.0;
    //}
    //if tm(0.05,0.1,i){
    //    f=880.0;
    //}
    //else{
    //    f=440.0;
    //}
    //o+=fm(t%1.0,1000.0,2.0)*0.3;
    //if i<sectosample(0.05){
    //    o+=fm(t,220.0,2.0)*0.5;
    //}
    //if i>sectosample(0.05)&&i<sectosample(0.08){
    //    o+=fm(t,880.0,2.0)*0.4;
    //}
    //if i>sectosample(0.08){
    //    o+=fm(t,440.0,1.25)*0.9;
    //}
}
