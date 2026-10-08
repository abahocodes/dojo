function longestSubarrayLimit(nums: number[], limit: number): number {
  const n = nums.length;
  // Deques as arrays with a moving head; indices are only appended once.
  const maxq: number[] = new Array(n);
  const minq: number[] = new Array(n);
  let maxHead = 0, maxTail = 0, minHead = 0, minTail = 0;
  let left = 0;
  let best = 0;
  for (let right = 0; right < n; right++) {
    const x = nums[right];
    while (maxTail > maxHead && nums[maxq[maxTail - 1]] < x) maxTail--;
    maxq[maxTail++] = right;
    while (minTail > minHead && nums[minq[minTail - 1]] > x) minTail--;
    minq[minTail++] = right;
    while (nums[maxq[maxHead]] - nums[minq[minHead]] > limit) {
      left++;
      if (maxq[maxHead] < left) maxHead++;
      if (minq[minHead] < left) minHead++;
    }
    best = Math.max(best, right - left + 1);
  }
  return best;
}
