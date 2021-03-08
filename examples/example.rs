use fmyers::{myers,DiffOp};

fn main(){
    let a = "abcabba";
    let b = "cbabac";
    let res = myers(a.as_bytes(),b.as_bytes());
    println!("{:?}",res);
}