#![allow(unused_variables)]
use rand::{Rng, rngs::ThreadRng, SeedableRng};
use rand_chacha::*;
use std::f32::consts::TAU as tau;
pub struct s{s:f32}
pub struct t{t:f32}
pub trait sample{}
impl t{
    pub fn sine(self,i:f32)->f32{
        (self.t*i).sin()
    }
}
pub trait signal<t>;
pub struct buffer{
    b:Vec<f32>,
    pub s:usize,
    r:usize,
    w:usize,
}
impl buffer{
    pub fn new(size:usize,ds:usize)->Self{
        let mut b = buffer{b:vec![],s:size,r:0,w:ds};
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
    pub fn wr(&mut self,i:f32,rngsz:usize){
        self.b[self.w%rngsz] = i;
        self.w+=1;
    }
    pub fn rd(&mut self,rngsz:usize)->f32{
        let o = self.b[self.r%rngsz];
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
pub fn stp(min:f32,max:f32,ns:f32)->f32{
    (max-min)/ns
}
pub fn lerp(i:f32,x1:f32,x2:f32,y1:f32,y2:f32)->f32{
    (y1+(i-x1))*((y2-y1)/(x2-x1))
}
pub fn lrp(min:f32,max:f32,amt:f32)->f32{
   min+(max-min)*amt
}
pub fn derlrp(min:f32,max:f32,ns:f32)->f32{
    min-(min+(max-min)*ns)
}
pub fn time(d:f32,i:f32)->bool{
    if i<sectosample(d){
        true
    }
    else{
        false
    }
}
pub fn t2i(t:f32)->f32{
    t*44100.0
}
pub fn i2t(i:f32)->f32{
    i/44100.0
}
pub fn tm(min:f32,max:f32,i:f32)->bool{
    if i>min && i<max{true}
    else{false}
}
pub fn invlrp(min:f32,max:f32,amt:f32)->f32{
    todo!()
}
pub fn noise(r:&mut ThreadRng)->f32{
    let x:f32 = r.gen();
    x
}
pub fn detns(s:u64)->f32{
    let mut x = rand_chacha::ChaCha8Rng::seed_from_u64(s);
    let r:f32 = x.gen_range(-1.0..1.0);
    r
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
pub fn bndz(i:f32)->f32{
    if i<0.0{0.0}else{i}}
trait boundmin{fn boundmin(self,min:f32)->Self;}
impl boundmin for f32{
    fn boundmin(self,min:f32)->Self{
        if self<min{min}
        else{self}
    }
}
trait boundmax{fn boundmax(self,max:f32)->Self;}
impl boundmax for f32{
    fn boundmax(self,max:f32)->Self{
        if self<max{max}
        else{self}
    }
}

pub trait bound{fn bound(self,min:Option<f32>,max:Option<f32>)->Self;}
impl bound for f32{
    fn bound(self,min:Option<f32>,max:Option<f32>)->Self{
        let mut r:f32 = 0.0;
        match min{
            Some(m)=>r = m,
            None=>r=self,
        }
        match max{
            Some(m)=>r=m,
            None=>r=self,
        }
        r
    }
}
pub trait square{fn square(self)->Self;}
impl square for f32{
    fn square(self)->Self{
        self.sin().signum()
    }
}
