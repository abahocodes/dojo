function smallestDistancePair(nums: number[], k: number): number {
  const sorted = [...nums].sort((a, b) => a - b);
  const n = sorted.length;
  const pairsWithin = (limit: number): number => {
    let count = 0;
    let left = 0;
    for (let right = 0; right < n; right++) {
      while (sorted[right] - sorted[left] > limit) left++;
      count += right - left;
    }
    return count;
  };

  let lo = 0;
  let hi = sorted[n - 1] - sorted[0];
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (pairsWithin(mid) >= k) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
