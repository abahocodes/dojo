function countBadPairs(nums) {
  const seen = new Map();
  let good = 0;
  for (let j = 0; j < nums.length; j++) {
    const key = nums[j] - j;
    const count = seen.get(key) || 0;
    good += count;
    seen.set(key, count + 1);
  }
  const n = nums.length;
  return (n * (n - 1)) / 2 - good;
}
