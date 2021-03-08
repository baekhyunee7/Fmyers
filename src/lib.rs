use itertools::Itertools;
use std::collections::HashMap;

pub fn fold_self<I, F>(mut i: I, f: F) -> Option<I::Item>
where
    I: Iterator,
    F: FnMut(I::Item, I::Item) -> I::Item,
{
    let first = i.next()?;
    Some(i.fold(first, f))
}

#[derive(Debug, Clone)]
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

impl PartialEq for DiffOp {
    fn eq(&self, other: &DiffOp) -> bool {
        matches!(
            (self, other),
            (DiffOp::Add { .. }, DiffOp::Add { .. })
                | (DiffOp::Remove { .. }, DiffOp::Remove { .. })
                | (DiffOp::Keep { .. }, DiffOp::Keep { .. })
        )
    }
}

pub fn myers<T>(src: &[T], dst: &[T]) -> Vec<DiffOp>
where
    T: PartialEq,
{
    let m = src.len();
    let n = dst.len();
    let total = m + n;
    let mut v: Vec<HashMap<isize, isize>> = vec![HashMap::new()];
    v[0].insert(1, 0);
    for d in 0..total {
        let d = d as isize;
        let mut map_cur: HashMap<isize, isize> = HashMap::new();
        let len = v.len();
        let map_pre = &v[len - 1];
        let mut k = -(d as isize);
        while k <= d as isize {
            let down = k == -d || k != d && abs(map_pre[&(k - 1)]) < abs(map_pre[&(k + 1)]);
            let kpre = if down { k + 1 } else { k - 1 };
            let x_start = abs(map_pre[&kpre]);
            // let y_start = x_start-kpre;
            let x_mid = if down { x_start } else { x_start + 1 };
            let y_mid = x_mid - k;
            let (mut x_end, mut y_end) = (x_mid, y_mid);
            while x_end < m as isize
                && y_end < n as isize
                && src[x_end as usize] == dst[y_end as usize]
            {
                x_end += 1;
                y_end += 1;
            }
            map_cur.insert(k, if down { x_end } else { -x_end });
            if x_end >= m as isize && y_end >= n as isize {
                v.push(map_cur);
                let mut res: Vec<DiffOp> = Vec::new();
                let mut k = (m - n) as isize;
                (2..v.len()).rev().for_each(|idx| {
                    let map_cur = &v[idx];
                    let map_pre = &v[idx - 1];
                    let mut x = map_cur[&k];
                    let down = x >= 0;
                    x = abs(x);
                    let y = x - k;
                    let k_pre = if down { k + 1 } else { k - 1 };
                    k = k_pre;
                    let x_pre = abs(map_pre[&k_pre]);
                    let y_pre = x_pre - k_pre;
                    if down {
                        if y - y_pre > 1 {
                            res.push(DiffOp::Keep {
                                old_idx: x_pre as usize,
                                len: x as usize - x_pre as usize,
                            });
                        }
                        res.push(DiffOp::Add {
                            old_idx: x_pre as usize,
                            new_idx: y_pre as usize,
                            len: 1,
                        });
                    } else {
                        if x - x_pre > 1 {
                            res.push(DiffOp::Keep {
                                old_idx: x_pre as usize + 1,
                                len: y as usize - y_pre as usize,
                            });
                        }
                        res.push(DiffOp::Remove {
                            old_idx: x_pre as usize,
                            len: 1,
                        });
                    }
                });
                return res
                    .into_iter()
                    .rev()
                    .group_by(|x| -> u8 {
                        match x {
                            DiffOp::Add { .. } => 0,
                            DiffOp::Remove { .. } => 1,
                            DiffOp::Keep { .. } => 2,
                        }
                    })
                    .into_iter()
                    .map(|(_, group)| {
                        fold_self(group, |a, b| match (a, b) {
                            (
                                DiffOp::Add {
                                    old_idx: o1,
                                    new_idx: n1,
                                    len: l1,
                                },
                                DiffOp::Add {
                                    old_idx: _,
                                    new_idx: _,
                                    len: l2,
                                },
                            ) => DiffOp::Add {
                                old_idx: o1,
                                new_idx: n1,
                                len: l1 + l2,
                            },
                            (
                                DiffOp::Remove {
                                    old_idx: o1,
                                    len: l1,
                                },
                                DiffOp::Remove {
                                    old_idx: _,
                                    len: l2,
                                },
                            ) => DiffOp::Remove {
                                old_idx: o1,
                                len: l1 + l2,
                            },
                            (
                                DiffOp::Keep {
                                    old_idx: o1,
                                    len: l1,
                                },
                                DiffOp::Keep {
                                    old_idx: _,
                                    len: l2,
                                },
                            ) => DiffOp::Keep {
                                old_idx: o1,
                                len: l1 + l2,
                            },
                            _ => panic!("diff error"),
                        })
                        .unwrap()
                    })
                    .collect::<Vec<DiffOp>>();
            }
            k += 2;
        }
        v.push(map_cur);
    }
    Vec::new()
}

fn abs(i: isize) -> isize {
    if i < 0 {
        -i
    } else {
        i
    }
}
