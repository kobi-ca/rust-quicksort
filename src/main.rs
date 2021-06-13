
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut v = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        println!("before {:?}", v);
        quicksort(&mut v);
        println!("after {:?}", v);
        assert!(is_sorted(&v));
    }
}

// https://stackoverflow.com/questions/51272571/how-do-i-check-if-a-slice-is-sorted
fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

fn sort_array(v: &mut [i32]) -> usize {
    println!("sorting {:?}", v);
    let lowest = 0;
    let highest = v.len() - 1;
    let pivot = v[highest];
    let mut lowest_idx = lowest;
    let mut lowest_running = lowest;
    while lowest_running < highest {
        if v[lowest_running] <= pivot {
            v.swap(lowest_running, lowest_idx);
            lowest_idx += 1;
        }
        lowest_running += 1;
    }
    println!("before last swap {:?}", v);
    v.swap(lowest_idx, highest);
    return lowest_idx;
}

fn quicksort_impl(v: &mut [i32]){
    if v.is_empty() {
        return;
    }
    let lowest = 0;
    let highest = v.len();
    let pivot_idx = sort_array(&mut v[lowest..highest]);
    println!("after sort_array {:?}, pivot idx {} lowest {} highest {}", v, pivot_idx, lowest, highest);
    quicksort_impl(&mut v[lowest..pivot_idx]);
    quicksort_impl(&mut v[pivot_idx + 1..highest]);
}

fn quicksort(v: &mut [i32]) {
    let len = v.len();
    quicksort_impl(&mut v[0..len])
}

fn main() {
    let v =&mut [1, 10, 2, 20, 3, 40, 50, 50, 100, 0];
    println!("before {:?}", v);
    quicksort(&mut v[..]);
    println!("after {:?}", v);
}
