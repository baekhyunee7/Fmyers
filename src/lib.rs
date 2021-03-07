use std::collections::HashMap;

pub fn myers<T>(src:&[T],dst:&[T])
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
            let down = k==- d || k!=d && map_pre[&(k-1)] < map_pre[&(k+1)];
            let kpre = if down{k+1}else{k-1};
            let x_start = map_pre[&kpre];
            let y_start = x_start-kpre;
            let x_mid = if down {x_start} else {x_start+1};
            let y_mid = x_mid-k;
            let (mut x_end,mut y_end) = (x_mid,y_mid);
            while x_end<m as isize&&y_end<n as isize&& src[x_end as usize] == dst[y_end as usize]{
                x_end+=1;
                y_end+=1;
            }
            // map_cur[&k] = x_end;
            map_cur.insert(k,x_end);
            if x_end>=m as isize&&y_end>=n as isize{
                println!("find {},{}",x_end,y_end);
                return;
            }
            k+=2;
        }
        v.push(map_cur);
    }
}