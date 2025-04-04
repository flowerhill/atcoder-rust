#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair<T>(T, T);

impl<T: Add<Output = T>> Add for Pair<T> {
    type Output = Pair<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Pair(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<Output = T>> Sub for Pair<T> {
    type Output = Pair<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Pair(self.0 - rhs.0, self.1 - rhs.1)
    }
}
