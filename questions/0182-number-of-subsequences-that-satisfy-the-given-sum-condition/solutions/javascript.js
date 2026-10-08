function numSubseq(nums, target) {
  const MOD = 1000000007;
  const a = [...nums].sort((x, y) => x - y);
  const n = a.length;
  const pow2 = new Array(n).fill(1);
  for (let i = 1; i < n; i++) pow2[i] = (pow2[i - 1] * 2) % MOD;
  let total = 0;
  let lo = 0;
  let hi = n - 1;
  while (lo <= hi) {
    if (a[lo] + a[hi] <= target) {
      total = (total + pow2[hi - lo]) % MOD;
      lo++;
    } else {
      hi--;
    }
  }
  return total;
}
