function checkSubarraySum(nums: number[], k: number): boolean {
  const first = new Map<number, number>([[0, -1]]);
  let rem = 0;
  for (let i = 0; i < nums.length; i++) {
    rem = (rem + nums[i]) % k;
    const j = first.get(rem);
    if (j !== undefined) {
      if (i - j >= 2) return true;
    } else {
      first.set(rem, i);
    }
  }
  return false;
}
