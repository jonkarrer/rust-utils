fn sort_arr(array: &Vec<i32>) -> Vec<i32> {
  let mut sorted_array = array.clone();

  for num in array {
    let mut comparison_results: Vec<bool> = Vec::new();

    for compare_num in array {
      if num > compare_num || num == compare_num {
        comparison_results.push(true);
      }
    }

    let sort_order_index = sorted_array.len() - comparison_results.len();
    sorted_array[sort_order_index] = *num;
  }

  return sorted_array;
}

fn main() {
  let spread = vec![50, 20, 4, 5, 8, 29, 100];
  let sorted_arr = sort_arr(&spread);
  dbg!(sorted_arr);
}
