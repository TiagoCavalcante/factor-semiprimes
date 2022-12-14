# factor-semiprime

An implementation of the Pollard's rho algorithm for factoring semiprime numbers

## Why?

[Semiprimes](https://en.wikipedia.org/wiki/Semiprime) (the product of two primes) are much used in cryptography, because multiplying huge primes is waaay faster than factoring the result. This repository contains an algorithm that attempts to do the hard part: factor huge semiprimes.

## How to use?

```sh
$ cargo install factor-semiprime
$ echo 18851959175571007 | factor-semiprime
18851959175571007 = 160097647 * 117752881
```

## How to generate semiprimes?

You can use [Symja](https://symjaweb.appspot.com/) or [Mathics](https://mathics.org/) to generate random primes and multiply them. Here is a code example that works on both projects:
```wl
(* Replace the range bellow with the desired range *)
r := RandomPrime[{10^8, 10^9}];
(* p is a pair of two random primes *)
p = {r, r};
(* Print the primes *)
Print[p];
(* Print the product of the two random primes *)
Print[Times @@ p];
```

## How fast is it?

```sh
$ time echo 437 | ./target/release/factor-semiprime
437 = 23 * 19

real    0m0.008s
user    0m0.003s
sys     0m0.007s

$ time echo 64786756484626223 | ./target/release/factor-semiprime
64786756484626223 = 222522227 * 291147349

real    0m0.026s
user    0m0.026s
sys     0m0.002s
$
```
