use rand::{Rng, rngs::ThreadRng};
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
pub fn force(i:f32,o:f32)->f32{
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

