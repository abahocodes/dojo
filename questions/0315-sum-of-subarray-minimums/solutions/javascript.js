function sumSubarrayMins(arr) {
  const MOD = 1000000007;
  const n = arr.length;
  const stack = [];
  let total = 0;
  for (let j = 0; j <= n; j++) {
    const cur = j < n ? arr[j] : 0;
    while (stack.length > 0 && arr[stack[stack.length - 1]] >= cur) {
      const i = stack.pop();
      const left = stack.length > 0 ? stack[stack.length - 1] : -1;
      // arr[i] * (i - left) * (j - i) <= 2.7 * 10^13, exact in a double.
      total = (total + arr[i] * (i - left) * (j - i)) % MOD;
    }
    stack.push(j);
  }
  return total;
}
