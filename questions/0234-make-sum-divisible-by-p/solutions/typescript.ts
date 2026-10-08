function minSubarrayRemove(nums: number[], p: number): number {
  let need = 0;
  for (const x of nums) need = (need + x) % p;
  if (need === 0) return 0;
  const latest = new Map<number, number>([[0, -1]]);
  let cur = 0;
  let best = nums.length;
  for (let j = 0; j < nums.length; j++) {
    cur = (cur + nums[j]) % p;
    const want = (cur - need + p) % p;
    const at = latest.get(want);
    if (at !== undefined) best = Math.min(best, j - at);
    latest.set(cur, j);
  }
  return best < nums.length ? best : -1;
}
