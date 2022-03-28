pub trait Except<T> {
    fn except(&self, other: &Self) -> Self;
}

pub trait Unite<T> {
    fn unite(&self, other: &Self) -> Self;
}

impl<T: Clone + PartialEq> Except<Vec<T>> for Vec<T> {
    fn except(&self, other: &Self) -> Self {
        if self.len() == 0 {
            return other.clone()
        } else if other.len() == 0 {
            return self.clone()
        }

        let mut res = vec![];
        for i in self {
            if !other.contains(i) {
                res.push(i.clone());
            }
        }

        for i in other {
            if !self.contains(i) {
                res.push(i.clone());
            }
        }

        res
    }
}

impl<T: Clone + PartialEq> Unite<Vec<T>> for Vec<T> {
    fn unite(&self, other: &Self) -> Self {
        if self.len() == 0 {
            return other.clone()
        } else if other.len() == 0 {
            return self.clone()
        }

        let mut res = vec![];
        for i in self {
            if !res.contains(i) {
                res.push(i.clone())
            }
        }

        for i in other {
            if !res.contains(i) {
                res.push(i.clone())
            }
        }

        res
    }
}