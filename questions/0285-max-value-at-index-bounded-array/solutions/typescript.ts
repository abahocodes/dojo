function maxValueAtIndex(n: number, index: number, maxSum: number): number {
  // Intermediate products above 2^53 only occur when the true cost is far
  // larger than maxSum, so double rounding never flips a comparison.
  const side = (v: number, len: number): number =>
    len >= v - 1 ? ((v - 1) * v) / 2 + (len - v + 1) : len * v - (len * (len + 1)) / 2;
  let lo = 1;
  let hi = maxSum;
  while (lo < hi) {
    const mid = Math.floor((lo + hi + 1) / 2);
    if (mid + side(mid, index) + side(mid, n - 1 - index) <= maxSum) lo = mid;
    else hi = mid - 1;
  }
  return lo;
}
