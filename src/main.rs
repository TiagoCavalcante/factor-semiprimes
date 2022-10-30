mod polland_rho;

fn main() {
  let mut buffer = String::with_capacity(128);
  std::io::stdin()
    .read_line(&mut buffer)
    .expect("failed to read from stdin");

  let integer =
    buffer.trim().parse().expect("input is not an integer");

  let (factor1, factor2) =
    polland_rho::factor_semiprime(integer);

  println!("{integer} = {factor1} * {factor2}");
}
