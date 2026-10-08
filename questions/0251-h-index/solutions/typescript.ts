function hIndex(citations: number[]): number {
  const n = citations.length;
  const buckets: number[] = new Array(n + 1).fill(0);
  for (const c of citations) buckets[Math.min(c, n)]++;
  let papers = 0;
  for (let h = n; h >= 0; h--) {
    papers += buckets[h];
    if (papers >= h) return h;
  }
  return 0;
}
