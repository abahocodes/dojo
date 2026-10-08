function robCircular(nums) {
  const line = (lo, hi) => {
    let prev = 0;
    let curr = 0;
    for (let i = lo; i < hi; i++) {
      const next = Math.max(curr, prev + nums[i]);
      prev = curr;
      curr = next;
    }
    return curr;
  };
  const n = nums.length;
  if (n === 1) return nums[0];
  return Math.max(line(0, n - 1), line(1, n));
}
