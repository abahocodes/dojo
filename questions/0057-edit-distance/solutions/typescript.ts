function editDistance(word1: string, word2: string): number {
  const m = word2.length;
  let prev = Array.from({ length: m + 1 }, (_, j) => j);
  for (let i = 1; i <= word1.length; i++) {
    const c1 = word1[i - 1];
    const curr = new Array<number>(m + 1).fill(0);
    curr[0] = i;
    for (let j = 1; j <= m; j++) {
      if (c1 === word2[j - 1]) curr[j] = prev[j - 1];
      else curr[j] = 1 + Math.min(prev[j], curr[j - 1], prev[j - 1]);
    }
    prev = curr;
  }
  return prev[m];
}
