function robCircular(nums: number[]): number {
  const line = (lo: number, hi: number): number => {
    let prev = 0;
    let curr = 0;
    for (let i = lo; i < hi; i++) {
      [prev, curr] = [curr, Math.max(curr, prev + nums[i])];
    }
    return curr;
  };

  const n = nums.length;
  if (n === 1) return nums[0];
  return Math.max(line(0, n - 1), line(1, n));
}
