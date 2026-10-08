function hIndexSorted(citations) {
  const n = citations.length;
  // Find the first index i where the n - i papers from i onward
  // all have at least n - i citations.
  let lo = 0;
  let hi = n;
  while (lo < hi) {
    const mid = (lo + hi) >> 1;
    if (citations[mid] >= n - mid) hi = mid;
    else lo = mid + 1;
  }
  return n - lo;
}
