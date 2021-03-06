fn myers<T>(src:&[T],dst:&[T])
    where T:PartialEq
{
       
}

pub struct VecWrapper{
    v_:Vec<Vec<usize>>
}

impl VecWrapper{
    pub fn new(m:usize,n:usize)->Self{
        let total = m+n;
        let mut v:Vec<Vec<usize>> = Vec::new();
        for i in 0..total{
            let mut inner:Vec<usize> = Vec::new();
            (0..total).map(|_|{inner.push(0)}).collect::<()>();
            v.push(inner);
        }
        Self{
            v_:v
        }
    }
}
