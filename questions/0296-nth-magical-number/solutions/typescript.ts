function nthMagicalNumber(n: number, a: number, b: number): number {
  const MOD = 1_000_000_007;
  const gcd = (x: number, y: number): number => (y === 0 ? x : gcd(y, x % y));
  const lcm = (a / gcd(a, b)) * b;
  let lo = Math.min(a, b);
  let hi = n * Math.min(a, b);
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    const count = Math.floor(mid / a) + Math.floor(mid / b) - Math.floor(mid / lcm);
    if (count >= n) hi = mid;
    else lo = mid + 1;
  }
  return lo % MOD;
}
