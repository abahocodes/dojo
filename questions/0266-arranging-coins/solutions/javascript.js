function arrangeCoins(n) {
  let lo = 0;
  let hi = Math.min(n, 94906266);
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    // mid * (mid + 1) is even and below 2^54, so it is exact in a double.
    if ((mid * (mid + 1)) / 2 <= n) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
