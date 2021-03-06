pub fn myers<T>(src:&[T],dst:&[T])
    where T:PartialEq
{
    let m  = src.len();
    let n = dst.len();
    let total = m+n;
    let mut v:Vec<Vec<usize>> = Vec::new();
    for d in 0..total{
        v.push(Vec::new());
        let mut k = - (d as isize);
        while k<=d as isize{
            println!("{}",k);
            k+=2;
        }
    }
}