# factor-semiprime

An implementation of the Pollard's rho algorithm for factoring semiprime numbers

## Why?

[Semiprimes](https://en.wikipedia.org/wiki/Semiprime) (the product of two primes) are much used in cryptography, because multiplying huge primes is waaay faster than factoring the result. This repository contains an algorithm that attempts to do the hard part: factor huge semiprimes.

## How to use?

```sh
$ cargo build --release
$ echo 18851959175571007 | ./target/release/factor-semiprime
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
