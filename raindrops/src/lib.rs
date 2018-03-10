pub fn raindrops(n: usize) -> String {
    let mut drops = String::new();
    if n % 3 == 0 {
      drops.push_str("Pling");
    }
    if n % 5 == 0 {
      drops.push_str("Plang");
    }
    if n % 7 == 0 {
      drops.push_str("Plong");
    }
    if drops.len() == 0 {
      drops.push_str(&n.to_string())
    }
    drops
}
