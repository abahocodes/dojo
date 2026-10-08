function longestCommonSubsequence(a: string, b: string): number {
  let prev = new Array<number>(b.length + 1).fill(0);
  for (const ca of a) {
    const curr = new Array<number>(b.length + 1).fill(0);
    for (let j = 1; j <= b.length; j++) {
      if (ca === b[j - 1]) curr[j] = prev[j - 1] + 1;
      else curr[j] = Math.max(prev[j], curr[j - 1]);
    }
    prev = curr;
  }
  return prev[b.length];
}
