fn subract(spread:&[f32]) -> f32 {
  let mut total = 0.0;

  for number in spread {
    total -= number
  }

  return total;
}

fn main() {
    let spread = [1.0, 5.2, 4.0];
    let sub = subract(&spread);
    println!("{sub}");
}
