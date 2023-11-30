#![allow(unused_variables)]
use rand::{Rng, rngs::ThreadRng};
use std::f32::consts::TAU as tau;
pub struct buffer{
    b:Vec<f32>,
    pub s:usize,
    r:usize,
    w:usize,
}
impl buffer{
    pub fn new(size:usize)->Self{
        let mut b = buffer{b:vec![],s:size,r:0,w:size};
        for x in 0..size{
            b.b.push(0.0);
        }
        b
    }
    pub fn write(&mut self,i:f32){
        self.b[self.w%self.s] = i;
        self.w+=1;
    }
    pub fn read(&mut self)->f32{
        let o = self.b[self.r%self.s];
        self.r+=1;
        o
    }
    pub fn varsize(&mut self,news:usize){
        if news ==0{
            self.s=1;
        }
        else{
            self.s = news;
        }
    }
}
pub trait norm{fn norm(self)->f32;}
impl norm for f32{
    fn norm(self)->f32{
       (self+1.0)/2.0
    }
}
//#[derive(Clone, Copy)]
//pub struct t{
//    pub t:f32,
//}
//impl t{
//    pub fn sine(self,f:f32)->f32{
//        (f*tau*self.t).sin()
//    }
//    pub fn square(self,f:f32)->f32{
//        self.sine(f).signum()
//    }
//    pub fn new()->Self{
//        Self {t:0.0}
//    }
//    pub fn upd(&mut self,new:f32)->&mut Self{
//        self.t = new;
//        self
//    }
//}
pub fn sin(f:f32,t:f32)->f32{
    (f*tau*t).sin()
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
pub fn slope(i:f32,x1:f32,x2:f32,y1:f32,y2:f32)->f32{
    if y2-y1 > 0.0 {
        i*((y2-y1)/(x2-x1))
    }
    else{
        (-((y2-y1)/(x2-x1))*i)+y1
    }
}

pub fn lerp(i:f32,x1:f32,x2:f32,y1:f32,y2:f32)->f32{
    (y1+(i-x1))*((y2-y1)/(x2-x1))
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
pub fn vibrato(t:f32,depth:f32,speed:f32)->f32{
    ((t*speed).sin())*depth
}
