fn find_greatest_num(spread: &[i32]) -> i32 {
  let mut greatest_num = 0;
  
  for num in spread {
    let mut comparison_results: Vec<bool> = Vec::new();

    // Compare current num with the other numbers in the array.
    for compare_num in spread {
      if num > compare_num || num == compare_num {
        comparison_results.push(true);
      }
    }
    
    // If the comparison array is full of true values equal to the size of the original array
    // ... then the current number is the greatest.
    if &comparison_results.len() == &spread.len() {
      greatest_num = *num;
      break
    }
  };
  return greatest_num;
}

fn main() {
  let greatest_number_in_array = find_greatest_num(&[3,20,7,11]);
  dbg!(greatest_number_in_array);
}
