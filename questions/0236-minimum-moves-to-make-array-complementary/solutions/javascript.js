function minMovesComplementary(nums, limit) {
  const n = nums.length;
  const delta = new Int32Array(2 * limit + 2);
  for (let i = 0; i < n / 2; i++) {
    const a = nums[i];
    const b = nums[n - 1 - i];
    const lo = Math.min(a, b);
    const hi = Math.max(a, b);
    delta[2] += 2;
    delta[lo + 1] -= 1;
    delta[hi + limit + 1] += 1;
    delta[a + b] -= 1;
    delta[a + b + 1] += 1;
  }
  let best = n;
  let moves = 0;
  for (let t = 2; t <= 2 * limit; t++) {
    moves += delta[t];
    best = Math.min(best, moves);
  }
  return best;
}
