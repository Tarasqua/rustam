trait SortStrategy {
    fn sort(&self, data: &mut [i32]);
}

struct BubbleSort;

struct QuickSort;

impl SortStrategy for BubbleSort {
    fn sort(&self, data: &mut [i32]) {
        let n = data.len();
        for i in 0..n {
            // Optimization: track if any swaps occurred in this pass
            let mut swapped = false;
            for j in 0..n - i - 1 {
                // Compare adjacent elements
                if data[j] > data[j + 1] {
                    // Swap elements if they are in the wrong order
                    data.swap(j, j + 1);
                    swapped = true;
                }
            }
            // If no elements were swapped in the inner loop, the list is sorted
            if !swapped {
                break;
            }
        }
    }
}

impl SortStrategy for QuickSort {
    fn sort(&self, data: &mut [i32]) {
        data.sort_unstable();
    }
}

fn sort_data(strategy: &dyn SortStrategy, data: &mut [i32]) {
    strategy.sort(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_bubble_sort() {
        let mut data = [3, 1, 2];
        let strategy = Box::new(BubbleSort);
        // INFO: * is used to dereference the boxed trait object to its underlying type: Box<BubbleSort> -> BubbleSort
        // INFO: &* is used to dereference the trait object itself: SortStrategy -> &dyn SortStrategy
        sort_data(&*strategy, &mut data);
        assert_eq!(data, [1, 2, 3]);
    }

    fn test_quick_sort() {
        let mut data = [3, 1, 2];
        let strategy = Box::new(QuickSort);
        // INFO: .as_ref() is analogous to &* but works with trait objects directly: SortStrategy -> &dyn SortStrategy
        sort_data(strategy.as_ref(), &mut data);
        assert_eq!(data, [1, 2, 3]);
    }
}
