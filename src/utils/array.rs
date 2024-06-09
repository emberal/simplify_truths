use std::cmp::max;

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

pub fn alternating_array(n: usize, mut skip: usize) -> Vec<bool> {
    skip = max(skip, 1);
    let mut array = vec![false; n];
    let mut cell_value = false;
    for (index, value) in array.iter_mut().enumerate() {
        if index % skip == 0 {
            cell_value = !cell_value;
        }
        *value = cell_value;
    }
    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alternating_array() {
        assert_eq!(alternating_array(4, 2), vec![true, true, false, false]);
        assert_eq!(alternating_array(5, 1), vec![true, false, true, false, true]);
    }

    #[test]
    fn test_alternating_array_0_skip() {
        assert_eq!(alternating_array(4, 0), vec![true, false, true, false]);
    }

    #[test]
    fn test_alternating_array_0_length() {
        assert_eq!(alternating_array(0, 2), vec![] as Vec<bool>);
    }

    #[test]
    fn test_alternating_array_skip_greater_than_length() {
        assert_eq!(alternating_array(4, 5), vec![true, true, true, true]);
    }

    #[test]
    fn test_distinct() {
        let mut vec = vec![1, 2, 3, 1, 2, 3];
        vec.distinct();
        assert_eq!(vec, vec![1, 2, 3]);
    }
}
