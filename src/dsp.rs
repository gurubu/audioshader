#![allow(unused_variables,unused_mut,unused_imports)]
use crate::fsf;
use crate::funk::*;
use rand::rngs::ThreadRng;
use std::f32::consts::TAU as tau;
pub struct rvrb8{
    d1:buffer,
    f1:buffer,
    d2:buffer,
    f2:buffer,
    d3:buffer,
    f3:buffer,
    d4:buffer,
    f4:buffer,
    d5:buffer,
    f5:buffer,
    d6:buffer,
    f6:buffer,
    d7:buffer,
    f7:buffer,
    d8:buffer,
    f8:buffer,
}

impl rvrb8{
    pub fn new(dur:usize)->Self{
      let mut f1 = buffer::new(dur,1);
      let mut d1 = buffer::new(dur,1000);
      let mut f2 = buffer::new(dur,2);
      let mut d2 = buffer::new(dur,2000);
      let mut f3 = buffer::new(dur,3);
      let mut d3 = buffer::new(dur,4000);
      let mut f4 = buffer::new(dur,4);
      let mut d4 = buffer::new(dur,8000);
      let mut f5 = buffer::new(dur,1);
      let mut d5 = buffer::new(dur,16000);
      let mut f6 = buffer::new(dur,2);
      let mut d6 = buffer::new(dur,32000);
      let mut f7 = buffer::new(dur,3);
      let mut d7 = buffer::new(dur,64000);
      let mut f8 = buffer::new(dur,4);
      let mut d8 = buffer::new(dur,128000);
      rvrb8{d1,f1,d2,f2,d3,f3,d4,f4,d5,f5,d6,f6,d7,f7,d8,f8}
    }
    pub fn write(&mut self,i:f32){
      self.d1.wr(i*0.9);
      self.f1.wr(self.d1.rd());
    }
}
pub fn sp(t:f32,f:f32)->f32{
    (f*tau*t).sin()*(-3.0*t).exp()
}
pub fn fm(t:f32,fc:f32,fm:f32)->f32{
    (fc*tau*t+1.0*((fc*fm*tau*t).sin())).sin()
}
pub fn ks(t:f32,b1:&mut buffer,b2:&mut buffer){
        let s1 = noise(t,0.001,0.0,0.3)+b2.rd();
    //delay
        b1.wr(s1*0.99);
        let d1 = b1.rd();
    //filter
        b2.wr(d1);
}
pub fn initreverb(dur:usize)->Vec<buffer>{
    let mut bv:Vec<buffer> = vec![];
    let mut bv:Vec<buffer> = vec![];
    bv.push(buffer::new(dur,10));
    bv.push(buffer::new(dur,20));
    bv.push(buffer::new(dur,30));
    bv.push(buffer::new(dur,40));
    bv.push(buffer::new(dur,50));
    bv.push(buffer::new(dur,60));
    bv
}
pub fn initrvrb8(dur:usize)->[buffer;8]{
    let mut rarr:[buffer;8]=[buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10),
                             buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10)];
    let mut filr:[buffer;8]=[buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10),
                             buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10),buffer::new(dur,10)];
    let mut rat = 10;
    for x in 0..8{
        rarr[x].varsize(rat+rat*x);
    }
    rarr
}
// pub fn rvrb8(barr:&mut[buffer;8],s:f32)->f32{
//     let mut res = 0.0;  
//     for x in 0..8{
//         barr[x].wr(s*0.9);
//         res += barr[x].rd();
//     }
//     res
// }
pub fn rvrb(bv:&mut Vec<buffer>,s:f32)->f32{
    let mut rs:f32=0.0;
    for x in 0..bv.len()-1{
        let r = bv[x].rd();
        bv[x].wr(s+r*0.9);
        rs+=bv[x].rd()*0.9;
    }
    rs
}
pub fn dlay(smp:f32,dur:usize)->f32{
    0.0
}
pub fn a() -> Vec<f32> {
    let mut v = vec![];
    let dur = sectosample(0.5) as usize;
    let frm = 1.0/44100.0;    
    let mut fir = buffer::new(dur,4);
    for x in 0..dur {
        let i = x as f32;
        let s = 44100.0;
        let t = i/s;
        // let c = ((t*20.0).sin()+1.0)*20000.0;
        // let c = (tri(t,200.0)+1000.0)*200.0;
        // let c = xp(8.0,t%2.0)*20000.0;
        let sq = square(t,880.0,0.05,0.0,0.2)+fir.rd();
        fir.wr(sq*0.99);
        // fir.varsize(c as usize);
        // del.wr(f1*0.99);
        // del.varsize(d as usize);
        // d2l.wr(f1*0.99);
        // fil.wr(fd);
        v.push(sq);
    }
    v
}
pub fn s(t: f32, d: f32, i: f32, r: &mut ThreadRng, b: &mut Vec<buffer>) -> f32 {
    let mut f: f32 = 440.0;
    let a: f32 = 0.2;
    let fs = stp(440.0, 220.0, d);
    let o = tri(t % 2000.0, 440.0) * 0.2 * xp(3.0, t);
    let d = tri(del(i, 0.01), 440.0) * 0.1 * xp(3.0, t);
    let s1 = sine(t, 440.0, 0.01, 0.00, 0.4);
    let s2 = sine(t, 880.0, 0.01, 0.01, 0.5);
    // let output = d + s1 + s2 + o + b[0].read();
    // b[0].write(output * 0.9);
    s1
    //o+d
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
    //let n  = detns(i)*(-3.0*t).exp()*0.3;
    //let d1 = detns(del(i, 600.0))*(-3.0*t).exp()*0.3;
    //let d1b= detns(del(i, 650.0))*(-3.0*t).exp()*0.1;
    //let d2 = detns(del(i,1000.0))*(-3.0*t).exp()*0.3;
    //let d2b= detns(del(i,1050.0))*(-3.0*t).exp()*0.1;
    //let d3 = detns(del(i,2000.0))*(-3.0*t).exp()*0.3;
    //let d3b= detns(del(i,2050.0))*(-3.0*t).exp()*0.1;
    //let d4 = detns(del(i,4000.0))*(-3.0*t).exp()*0.3;
    //let d4b= detns(del(i,4050.0))*(-3.0*t).exp()*0.1;
    //n+d1+d2+d3+d4+
    //d1b+d2b+d3b+d4b

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
