function numOfWays(nums: number[]): number {
  const MOD = 1000000007;
  // a * b mod MOD without exceeding 2^53: split b into 16-bit halves.
  const mul = (a: number, b: number): number =>
    (((a * (b >>> 16)) % MOD) * 65536 + a * (b & 65535)) % MOD;
  const power = (base: number, exp: number): number => {
    let result = 1;
    while (exp > 0) {
      if (exp & 1) result = mul(result, base);
      base = mul(base, base);
      exp = Math.floor(exp / 2);
    }
    return result;
  };

  const n = nums.length;
  const left = new Int32Array(n + 1);
  const right = new Int32Array(n + 1);
  const root = nums[0];
  for (let i = 1; i < n; i++) {
    const v = nums[i];
    let cur = root;
    while (true) {
      if (v < cur) {
        if (left[cur] === 0) { left[cur] = v; break; }
        cur = left[cur];
      } else {
        if (right[cur] === 0) { right[cur] = v; break; }
        cur = right[cur];
      }
    }
  }

  const fact: number[] = new Array(n + 1).fill(1);
  for (let i = 1; i <= n; i++) fact[i] = mul(fact[i - 1], i);
  const invFact: number[] = new Array(n + 1).fill(1);
  invFact[n] = power(fact[n], MOD - 2);
  for (let i = n; i > 0; i--) invFact[i - 1] = mul(invFact[i], i);

  const size: number[] = new Array(n + 1).fill(0);
  const ways: number[] = new Array(n + 1).fill(1);
  for (let i = n - 1; i >= 0; i--) {
    const v = nums[i];
    const l = left[v];
    const r = right[v];
    size[v] = size[l] + size[r] + 1;
    const interleave = mul(mul(fact[size[l] + size[r]], invFact[size[l]]), invFact[size[r]]);
    ways[v] = mul(mul(interleave, ways[l]), ways[r]);
  }
  return (ways[root] - 1 + MOD) % MOD;
}
