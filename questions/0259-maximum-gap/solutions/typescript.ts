function maximumGap(nums: number[]): number {
  const n = nums.length;
  if (n < 2) return 0;
  let lo = nums[0], hi = nums[0];
  for (const v of nums) {
    lo = Math.min(lo, v);
    hi = Math.max(hi, v);
  }
  if (lo === hi) return 0;
  const size = Math.max(1, Math.floor((hi - lo) / (n - 1)));
  const count = Math.floor((hi - lo) / size) + 1;
  const bucketMin: number[] = new Array(count).fill(Infinity);
  const bucketMax: number[] = new Array(count).fill(-1);
  for (const v of nums) {
    const b = Math.floor((v - lo) / size);
    if (v < bucketMin[b]) bucketMin[b] = v;
    if (v > bucketMax[b]) bucketMax[b] = v;
  }
  let best = 0, prev = lo;
  for (let b = 0; b < count; b++) {
    if (bucketMax[b] < 0) continue; // empty
    best = Math.max(best, bucketMin[b] - prev);
    prev = bucketMax[b];
  }
  return best;
}
