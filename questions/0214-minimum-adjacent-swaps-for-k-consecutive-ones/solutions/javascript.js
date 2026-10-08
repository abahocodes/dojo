function minMovesKOnes(nums, k) {
  const q = [];
  for (let i = 0; i < nums.length; i++) {
    if (nums[i] === 1) q.push(i - q.length);
  }
  const prefix = [0];
  for (const v of q) prefix.push(prefix[prefix.length - 1] + v);
  let best = Infinity;
  for (let lo = 0; lo + k <= q.length; lo++) {
    const hi = lo + k - 1;
    const mid = lo + Math.floor(k / 2);
    const m = q[mid];
    const cost = m * (mid - lo) - (prefix[mid] - prefix[lo])
      + (prefix[hi + 1] - prefix[mid + 1]) - m * (hi - mid);
    if (cost < best) best = cost;
  }
  return best;
}
