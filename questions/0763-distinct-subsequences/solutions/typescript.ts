function numDistinct(s: string, t: string): number {
  const m = t.length;
  // ways[j] = number of ways to form t[:j] from the part of s read so far.
  // Prefix counts that never reach the answer can grow past 2^53 and lose
  // precision, but they never feed into ways[m], which stays exact.
  const ways: number[] = new Array(m + 1).fill(0);
  ways[0] = 1;
  for (const ch of s) {
    // Go right to left so ways[j - 1] still holds the previous row's value.
    for (let j = m; j >= 1; j--) {
      if (t[j - 1] === ch) ways[j] += ways[j - 1];
    }
  }
  return ways[m];
}
