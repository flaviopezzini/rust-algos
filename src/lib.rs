use std::fmt::Debug;

mod b_rand;

pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut [T]) {
    for p in 0..v.len() {
        let mut sorted = true;
        for i in 0..(v.len() - 1) - p {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        println!("{:?}", v);
        if sorted {
            return;
        }
    }
}

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    // sort the left half,
    // sort the right half, O ( n * log n)
    // bring the sorted halfs together

    println!("V is {:?}", v);

    if v.len() <= 1 {
        return v;
    }

    let mut res = Vec::with_capacity(v.len());
    let right = v.split_off(v.len() / 2);

    let left = merge_sort(v);
    let right = merge_sort(right);

    // bring them together again
    let mut left_it = left.into_iter();
    let mut right_it = right.into_iter();
    let mut left_peek = left_it.next();
    let mut right_peek = right_it.next();

    loop {
        match left_peek {
            Some(ref left_val) => match right_peek {
                Some (ref right_val) => {
                    if right_val < left_val {
                        res.push(right_peek.take().unwrap());
                        right_peek = right_it.next();
                    } else {
                        res.push(left_peek.take().unwrap());
                        left_peek = left_it.next();
                    }
                }
                None => {
                    res.push(left_peek.take().unwrap());
                    res.extend(left_it);
                    return res;
                }
            }
            None => {
                if let Some(right_val) = right_peek {
                    res.push(right_val);
                }
                res.extend(right_it);
                return res;
            }
        }
    }
}

pub fn pivot<T: PartialOrd>(v: &mut [T]) -> usize {
    let mut p = b_rand::rand(v.len());
    v.swap(p, 0);
    p = 0;

    for i in 1..v.len() {
        if v[i] < v[p] {
            v.swap(p+1, i);
            v.swap(p, p+1);
            p += 1;
        }
    }
    p
}

pub fn quick_sort<T: PartialOrd + Debug>(v:&mut[T]) {
    if v.len() <= 1 {
        return;
    }

    let p = pivot(v);

    println!("{:?}", v);

    let (a, b) = v.split_at_mut(p);
    quick_sort(a);
    quick_sort(&mut b[1..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_bubble_sort_simple_array() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn should_merge_sort_simple_array() {
        let v = vec![4, 6, 1, 8, 11, 13, 3];
        let sorted = merge_sort(v);
        assert_eq!(sorted, vec![1, 3, 4, 6, 8, 11, 13]);
    }

    #[test]
    fn test_pivot() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        let p = pivot(&mut v);

        for x in 00..v.len() {
            assert!((v[x] < v[p]) == (x < p));
        }
    }

    #[test]
    fn should_quick_sort_simple_array() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13]);
    }
}
