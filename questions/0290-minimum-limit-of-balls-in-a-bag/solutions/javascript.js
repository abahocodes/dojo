function minimumSize(nums, maxOperations) {
  let lo = 1;
  let hi = 0;
  for (const b of nums) if (b > hi) hi = b;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    let ops = 0;
    for (const b of nums) {
      ops += Math.floor((b - 1) / mid);
      if (ops > maxOperations) break;
    }
    if (ops <= maxOperations) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
