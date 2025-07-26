use crate::vec_stack::Stack;

fn partition<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot_index = high;
    let mut i = low;
    for j in low..high {
        if arr[j] <= arr[pivot_index] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}
fn quicksort_stack<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mut stack = Stack::new();

    stack.push((0, arr.len() - 1));

    while let Some((low, high)) = stack.pop() {
        if low < high {
            let p = partition(arr, low, high);

            if p > low {
                stack.push((low, p - 1));
            }

            if p < high {
                stack.push((p + 1, high));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut numbers = vec![9, 4, 1, 7, 3, 8, 2, 6, 5];
        println!("Unsorted: {:?}", numbers);

        quicksort_stack(&mut numbers);
        println!("Sorted:   {:?}", numbers);
    }
}
