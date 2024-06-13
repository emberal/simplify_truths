#[macro_export]
macro_rules! set {
    () => { std::collections::HashSet::new() };
    ($($x:expr),*) => {
        {
            let mut temp_set = std::collections::HashSet::new();
            $(
                temp_set.insert($x);
            )*
            temp_set
        }
    };
}

#[macro_export]
macro_rules! matrix {
    ($($($x:expr),*);*) => {
        {
            let mut temp_vec = vec![];
            {} // Needed to avoid clippy warning
            $(
                temp_vec.push(vec![$($x),*]);
            )*
            temp_vec
        }
    };
    ($x:expr; $m:expr => $n:expr) => {
        vec![vec![$x; $m]; $n]
    };
}

#[macro_export]
macro_rules! map {
    () => { std::collections::HashMap::new() };
    ($($k:expr => $v:expr),*) => {
        {
            let mut temp_map = std::collections::HashMap::new();
            $(
                temp_map.insert($k, $v);
            )*
            temp_map
        }
    };
}


pub trait Distinct {
    fn distinct(&mut self);
}

impl<T: PartialEq + Clone> Distinct for Vec<T> {
    fn distinct(&mut self) {
        *self = self.iter()
            .fold(vec![], |mut acc, x| {
                if !acc.contains(x) {
                    acc.push(x.clone());
                }
                acc
            });
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct() {
        let mut vec = vec![1, 2, 3, 1, 2, 3];
        vec.distinct();
        assert_eq!(vec, vec![1, 2, 3]);
    }
}
