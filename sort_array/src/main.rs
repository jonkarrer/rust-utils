fn sort(unsorted: &Vec<i32>) -> Vec<i32> {
    let mut sorted = unsorted.clone();

    for value in unsorted {
        let mut comparison_results: Vec<bool> = Vec::new();

        for comparison_value in unsorted {
            if value > comparison_value || value == comparison_value {
                comparison_results.push(true);
            }
        }

        let order = sorted.len() - comparison_results.len();
        sorted[order] = *value;
    }

    return sorted;
}

fn main() {
    let spread = vec![50, 20, 4, 5, 8, 29, 100];
    let sorted_arr = sort(&spread);
    dbg!(sorted_arr);
}
