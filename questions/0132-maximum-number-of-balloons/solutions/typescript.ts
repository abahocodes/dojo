function maxNumberOfBalloons(text: string): number {
  const have: number[] = new Array(26).fill(0);
  for (let i = 0; i < text.length; i++) have[text.charCodeAt(i) - 97]++;
  const need: number[] = new Array(26).fill(0);
  for (const c of "balloon") need[c.charCodeAt(0) - 97]++;
  let best = Infinity;
  for (let i = 0; i < 26; i++) {
    if (need[i] > 0) best = Math.min(best, Math.floor(have[i] / need[i]));
  }
  return best;
}
