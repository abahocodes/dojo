function splitArray(nums, k) {
  const piecesNeeded = (cap) => {
    let pieces = 1;
    let current = 0;
    for (const x of nums) {
      if (current + x > cap) {
        pieces++;
        current = x;
      } else {
        current += x;
      }
    }
    return pieces;
  };

  let lo = 0;
  let hi = 0;
  for (const x of nums) {
    lo = Math.max(lo, x);
    hi += x;
  }
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (piecesNeeded(mid) <= k) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
