pub fn reverse(input: &str) -> String {
  let mut rev = String::new();
  for c in input.chars().rev() {
    rev.push(c)
  }
  rev
  // Apparently same as:
  // input.chars().rev().collect()
  // daaaymn do i even rust bro
}
