use crate::integer::Integer;

// 二分探索
// 左をfalse, 右をtrueとして、条件を満たす最小の値を探す
pub fn bisect<T: Integer>(l: T, r: T, mut f: impl FnMut(&T) -> bool) -> (T, T) {
    let (mut ng, mut ok) = (l, r);
    while ok > ng + T::ONE {
        let mid = ng + (ok - ng) / T::TWO;
        *if f(&mid) { &mut ok } else { &mut ng } = mid;
    }
    (ng, ok)
}

pub trait LowerBound<T> {
    type Item: Ord;
    fn lower_bound(&self, x: &T) -> usize;
}

impl<T: Ord> LowerBound<T> for [T] {
    type Item = T;
    fn lower_bound(&self, x: &T) -> usize {
        let res = bisect(0, self.len(), |&i| unsafe { self.get_unchecked(i) < x });
        res.0
    }
}

pub trait UpperBound<T> {
    type Item: Ord;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> UpperBound<T> for [T] {
    type Item = T;
    fn upper_bound(&self, x: &T) -> usize {
        let res = bisect(0, self.len(), |&i| unsafe { self.get_unchecked(i) <= x });
        res.1
    }
}
