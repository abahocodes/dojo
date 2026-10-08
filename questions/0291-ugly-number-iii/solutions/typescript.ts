function nthUglyNumber(n: number, a: number, b: number, c: number): number {
  const LIMIT = 2e9;
  const gcd = (p: number, q: number): number => {
    while (q) [p, q] = [q, p % q];
    return p;
  };
  // p / gcd * q, capped at LIMIT + 1 before it can lose precision.
  const lcm = (p: number, q: number): number => {
    const r = p / gcd(p, q);
    return r > (LIMIT + 1) / q ? LIMIT + 1 : Math.min(r * q, LIMIT + 1);
  };
  const ab = lcm(a, b);
  const ac = lcm(a, c);
  const bc = lcm(b, c);
  const abc = lcm(ab, c);
  const f = Math.floor;
  const count = (x: number): number =>
    f(x / a) + f(x / b) + f(x / c) - f(x / ab) - f(x / ac) - f(x / bc) + f(x / abc);
  let lo = 1;
  let hi = LIMIT;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (count(mid) >= n) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
