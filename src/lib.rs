use std::collections::HashMap;

pub enum DiffOp{
    Add{old_idx:isize,new_idx:isize,len:isize},
    Remove{old_idx:isize,len:isize},
    Keep{old_idx:isize,len:isize}
}

pub fn myers<T>(src:&[T],dst:&[T])->Vec<DiffOp>
    where T:PartialEq
{
    let m  = src.len();
    let n = dst.len();
    let total = m+n;
    let mut v:Vec<HashMap<isize,isize>> = Vec::new();
    v.push(HashMap::new());
    &v[0].insert(1,0);
    for d in 0..total{
        let d = d as isize;
        let mut map_cur:HashMap<isize,isize> = HashMap::new();
        let len = v.len();
        let map_pre = &v[len-1];
        let mut k = - (d as isize);
        // k = x-y -> y = x-k
        while k<=d as isize{
            let down = k==- d || k!=d && abs(map_pre[&(k-1)]) < abs(map_pre[&(k+1)]);
            let kpre = if down{k+1}else{k-1};
            let x_start = abs(map_pre[&kpre]);
            let y_start = x_start-kpre;
            let x_mid = if down {x_start} else {x_start+1};
            let y_mid = x_mid-k;
            let (mut x_end,mut y_end) = (x_mid,y_mid);
            while x_end<m as isize&&y_end<n as isize&& src[x_end as usize] == dst[y_end as usize]{
                x_end+=1;
                y_end+=1;
            }
            // map_cur[&k] = x_end;
            map_cur.insert(k,if down{x_end}else{-x_end});
            if x_end>=m as isize&&y_end>=n as isize{
                v.push(map_cur);
                let mut res:Vec<DiffOp> = Vec::new();
                let mut k = (m-n) as isize;
                (2..v.len()).rev().map(|idx|{
                    let map_cur = &v[idx];
                    let map_pre = &v[idx-1];
                    let mut x = map_cur[&k];
                    let down = x>=0;
                    x = abs(x);
                    let y = x-k;
                    if down{
                        let k_pre = k+1;
                        k = k_pre;
                        let x_pre = abs(map_pre[&k_pre]);
                        let y_pre = x_pre-k_pre;
                        if y - y_pre>1{
                            res.push(DiffOp::Add{old_idx:x_pre,new_idx:y-1,len:1});
                            res.push(DiffOp::Keep{old_idx:x_pre,len:x-x_pre});
                        }else{
                            res.push(DiffOp::Add{old_idx:x_pre,new_idx:y-1,len:1});
                        }
                    }else{
                        let k_pre = k-1;
                        k = k_pre;
                        let x_pre = abs(map_pre[&k_pre]);
                        let y_pre = x_pre-k_pre;
                        if x - x_pre>1{
                            res.push(DiffOp::Remove{old_idx:x-1,len:1});
                            res.push(DiffOp::Keep{old_idx:x_pre,len:y-y_pre});
                        }else{
                            res.push(DiffOp::Remove{old_idx:x-1,len:1});
                        }
                    }
                }).collect::<()>();
                return res;
            }
            k+=2;
        }
        v.push(map_cur);
    }
    Vec::new()
}

fn abs(i:isize)->isize{
    if i<0{-i}else{i}
}