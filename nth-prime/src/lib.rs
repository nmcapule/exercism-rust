pub fn nth(n: u32) -> Option<u32> {
    return match n {
        0 => None,
        1 => Some(2),
        2 => Some(3),
        _ => {
            let mut k = (2..120000)
                .collect::<Vec<u32>>();
            let mut curr: u32 = 0;
            for _ in 0..n {
              curr = *(&k.get(0)).unwrap();
              k = k.iter()
                  .filter(|&x| (x % curr != 0))
                  .map(|&x| (x))
                  .collect();
            }
            Some(curr)
        }
    }
}
