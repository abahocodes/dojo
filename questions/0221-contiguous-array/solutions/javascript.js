function findMaxLength(nums) {
  const n = nums.length;
  // balance ranges over [-n, n]; store first index at balance + n
  const first = new Array(2 * n + 1).fill(-2);
  first[n] = -1;
  let balance = 0;
  let best = 0;
  for (let i = 0; i < n; i++) {
    balance += nums[i] === 1 ? 1 : -1;
    const slot = balance + n;
    if (first[slot] !== -2) best = Math.max(best, i - first[slot]);
    else first[slot] = i;
  }
  return best;
}
