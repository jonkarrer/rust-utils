fn find_largest_number(arr: &[i32]) -> i32 {
  // loop through and compare each number to each number
  // loop through
  let mut largest_number = 0;
  
  for number in arr {
    // compare number to all numbers
    let mut truth_arr: Vec<bool> = Vec::new();
    for comparison in arr {
      if number > comparison || number == comparison {
        truth_arr.push(true);
      }
    }
    let truth_arr_length = &truth_arr.len();
    let arr_length = &arr.len();

    if truth_arr_length == arr_length {
      largest_number = *number;
      break
    }
  };
  return largest_number;
}

fn main() {
  let highest_chunk_possible = find_largest_number(&[3,20,7,11]);
  dbg!(highest_chunk_possible);
}
