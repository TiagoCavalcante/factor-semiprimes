use rand::Rng;

fn gcd(mut a: i128, mut b: i128) -> i128 {
  while a % b != 0 {
    let r = a % b;
    a = b;
    b = r;
  }

  b
}

// Factor the number n into 2 prime factors using the
// Pollards-Rho factorization algorithm.
pub fn factor_semiprime(n: i128) -> (i128, i128) {
  let mut rng = rand::thread_rng();

	// Loop while the factor is n.
  loop {
    // Pick x in range [2, n).
    let mut x = (rng.gen::<i128>() % (n - 2)) + 2;
    let mut y = x;

    let mut factor = 1;

    while factor == 1 {
      // Tortoise Move: x_(i+1) = f(x_i)
      x = (x * x + 1) % n;

      // Hare Move: y_(i+1) = f(f(y_i))
      y = (y * y + 1) % n;
      y = (y * y + 1) % n;

      factor = gcd((x - y).abs(), n);
    }

    if factor != n {
      return (factor, n / factor);
    }
  }
}
