pub fn collatz(mut n: u64) -> Option<u64> {
  let mut steps = 0;
  while n!=1{
      if n == 0 {
          return None;
      } else {
          match n {
          _ if n % 2 == 0 => n /= 2,
          _ => n = 3 * n + 1,
          }
          steps += 1;
      }
  }
  Some(steps)
}

