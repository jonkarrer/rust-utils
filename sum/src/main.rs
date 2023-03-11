fn sum(spread: &[f32]) -> f32 {
    let mut total: f32 = 0.0;

    for number in spread {
        total += number;
    }

    return total;
}

fn main() {
    let spread = &[1.0, 4.0, 5.0, 6.0];

    let sum: f32 = sum(spread);

    println!("{sum}");
}
