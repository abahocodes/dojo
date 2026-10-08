function maxSubArrayLen(nums, k) {
  const first = new Map([[0, -1]]);
  let prefix = 0;
  let best = 0;
  for (let i = 0; i < nums.length; i++) {
    prefix += nums[i];
    const j = first.get(prefix - k);
    if (j !== undefined) best = Math.max(best, i - j);
    if (!first.has(prefix)) first.set(prefix, i);
  }
  return best;
}
