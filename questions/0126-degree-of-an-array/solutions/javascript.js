function findShortestSubArray(nums) {
  const first = new Map();
  const count = new Map();
  let degree = 0;
  let best = 0;
  for (let i = 0; i < nums.length; i++) {
    const x = nums[i];
    if (!first.has(x)) first.set(x, i);
    const c = (count.get(x) || 0) + 1;
    count.set(x, c);
    const span = i - first.get(x) + 1;
    if (c > degree || (c === degree && span < best)) {
      degree = c;
      best = span;
    }
  }
  return best;
}
