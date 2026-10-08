function maxScoreSplit(s: string): number {
  let score = 0;
  for (const c of s) if (c === "1") score++;
  let best = 0;
  for (let i = 0; i < s.length - 1; i++) {
    score += s[i] === "0" ? 1 : -1;
    best = Math.max(best, score);
  }
  return best;
}
