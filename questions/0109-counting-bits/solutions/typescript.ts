function countBits(n: number): number[] {
  const bits = new Array<number>(n + 1).fill(0);
  for (let i = 1; i <= n; i++) {
    bits[i] = bits[i >> 1] + (i & 1);
  }
  return bits;
}
