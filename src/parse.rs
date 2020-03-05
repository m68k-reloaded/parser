pub fn parse(tokens: Scanner) {
    for token in &tokens {
        println!("Got token {:?}.", token);
    }
}
