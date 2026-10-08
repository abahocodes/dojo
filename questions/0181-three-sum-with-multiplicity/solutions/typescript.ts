function threeSumMulti(arr: number[], target: number): number {
  const MOD = 1000000007;
  const c: number[] = new Array(101).fill(0);
  for (const v of arr) c[v]++;
  // Every product below stays under 2.7e10, well inside exact double range.
  let total = 0;
  for (let x = 0; x <= 100; x++) {
    for (let y = x; y <= 100; y++) {
      const z = target - x - y;
      if (z < y || z > 100) continue;
      let ways: number;
      if (x === y && y === z) ways = (c[x] * (c[x] - 1) * (c[x] - 2)) / 6;
      else if (x === y) ways = ((c[x] * (c[x] - 1)) / 2) * c[z];
      else if (y === z) ways = (c[x] * c[y] * (c[y] - 1)) / 2;
      else ways = c[x] * c[y] * c[z];
      total = (total + ways) % MOD;
    }
  }
  return total;
}
