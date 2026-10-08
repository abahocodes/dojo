function getHint(secret: string, guess: string): string {
  let bulls = 0, cows = 0;
  const bal: number[] = new Array(10).fill(0);
  for (let i = 0; i < secret.length; i++) {
    const a = secret.charCodeAt(i) - 48;
    const b = guess.charCodeAt(i) - 48;
    if (a === b) { bulls++; continue; }
    if (bal[a] < 0) cows++;
    if (bal[b] > 0) cows++;
    bal[a]++;
    bal[b]--;
  }
  return `${bulls}A${cows}B`;
}
