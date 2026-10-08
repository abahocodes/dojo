function isInterleave(s1: string, s2: string, s3: string): boolean {
  const m = s1.length;
  const n = s2.length;
  if (m + n !== s3.length) return false;
  const ok: boolean[] = new Array(n + 1).fill(false);
  for (let i = 0; i <= m; i++) {
    for (let j = 0; j <= n; j++) {
      if (i === 0 && j === 0) {
        ok[j] = true;
        continue;
      }
      const c = s3[i + j - 1];
      const fromS1 = i > 0 && ok[j] && s1[i - 1] === c;
      const fromS2 = j > 0 && ok[j - 1] && s2[j - 1] === c;
      ok[j] = fromS1 || fromS2;
    }
  }
  return ok[n];
}
