function mySqrt(x: number): number {
  let lo = 0;
  let hi = Math.min(x, 67108864); // 2^26
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    if (mid * mid <= x) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
