function totalStrength(strength: number[]): number {
  const MOD = 1000000007n;
  const n = strength.length;
  const left: number[] = new Array(n).fill(-1);
  const right: number[] = new Array(n).fill(n);
  const stack: number[] = [];
  for (let i = 0; i < n; i++) {
    while (stack.length > 0 && strength[stack[stack.length - 1]] >= strength[i]) {
      right[stack.pop()!] = i; // first smaller-or-equal to the right
    }
    left[i] = stack.length > 0 ? stack[stack.length - 1] : -1; // first strictly smaller to the left
    stack.push(i);
  }
  // pp[k] = P[0] + ... + P[k-1], where P[k] = strength[0] + ... + strength[k-1]
  const pp: bigint[] = new Array(n + 2).fill(0n);
  let p = 0n;
  for (let k = 0; k <= n; k++) {
    pp[k + 1] = (pp[k] + p) % MOD;
    if (k < n) p = (p + BigInt(strength[k])) % MOD;
  }
  let total = 0n;
  for (let i = 0; i < n; i++) {
    const l = left[i];
    const r = right[i];
    const plus = (BigInt(i - l) * (pp[r + 1] - pp[i + 1])) % MOD;
    const minus = (BigInt(r - i) * (pp[i + 1] - pp[l + 1])) % MOD;
    const span = (((plus - minus) % MOD) + MOD) % MOD;
    total = (total + BigInt(strength[i]) * span) % MOD;
  }
  return Number(total);
}
