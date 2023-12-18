use std::{fs::File, io::Read};

use cbqn::{self,eval,BQN};
pub fn bqntosound()->Vec<f32>{
 let mut file = File::open("music.bqn").unwrap();
 let mut bqnstr = String::new();
 file.read_to_string(&mut bqnstr).unwrap();
 let bqn = BQN!(&bqnstr).unwrap().to_f64_vec().unwrap();
 let mut vout:Vec<f32>=vec![];
 for x in 0..bqn.len(){
  vout.push(bqn[x] as f32);
 }
 vout
}
