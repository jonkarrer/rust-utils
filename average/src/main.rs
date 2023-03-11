fn average(spread: &[f32]) -> f32 {
    // Sum the spread
    let mut sum = 0.0;
    for number in spread {
        sum += number
    }

    // Divide by the length of the spread
    sum / spread.len() as f32
}

fn main() {
    let result = average(&[5.8, 7.6, 8.9]);
    println!("{result}");
}
