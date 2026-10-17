function myPow(x, n) {
  // Binary exponentiation: square the base and halve the exponent,
  // multiplying the base into the result whenever the low bit is set.
  // Plain division by 2 (not >>) keeps -(-2^31) exact.
  if (n < 0) {
    x = 1 / x;
    n = -n;
  }
  let result = 1;
  while (n > 0) {
    if (n % 2 === 1) result *= x;
    x *= x;
    n = Math.floor(n / 2);
  }
  return result;
}
