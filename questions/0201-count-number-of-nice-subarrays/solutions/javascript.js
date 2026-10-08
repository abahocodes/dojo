function numberOfNiceSubarrays(nums, k) {
  const seen = new Array(nums.length + 1).fill(0);
  seen[0] = 1;
  let odds = 0;
  let total = 0;
  for (const x of nums) {
    odds += x & 1;
    if (odds >= k) total += seen[odds - k];
    seen[odds]++;
  }
  return total;
}
