# Fmyers

* myers() return Vec<DiffOp>
```
pub enum DiffOp {
    Add {
        old_idx: usize,
        new_idx: usize,
        len: usize,
    },
    Remove {
        old_idx: usize,
        len: usize,
    },
    Keep {
        old_idx: usize,
        len: usize,
    },
}
```

### example
```
use fmyers::{myers, DiffOp};

fn main() {
    let a = "abcabba";
    let b = "cbabac";
    let res = myers(a.as_bytes(), b.as_bytes());    
    res.iter().for_each(|diff| match *diff {
        DiffOp::Add {
            new_idx: n, len: l, ..
        } => println!("+{}", &b[n..n + l]),
        DiffOp::Remove { old_idx: o, len: l } => println!("-{}", &a[o..o + l]),
        DiffOp::Keep { old_idx: o, len: l } => println!("{}", &a[o..o + l]),
    })
}

//output:
-ab
c
+b
ab
-b
a
+c
```
