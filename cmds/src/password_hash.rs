fn main() {
    let input = std::env::args().nth(1).expect("no input");
    let hashed = iam_common::password::hash(&input).expect("failed to hash");
    println!("{hashed}");
}
