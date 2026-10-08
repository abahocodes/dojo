function wordBreak(s: string, words: string[]): boolean {
  const vocab = new Set(words);
  const lengths = [...new Set(words.map((w) => w.length))].sort((a, b) => a - b);
  const ok = new Array<boolean>(s.length + 1).fill(false);
  ok[0] = true;
  for (let i = 1; i <= s.length; i++) {
    for (const length of lengths) {
      if (length > i) break;
      if (ok[i - length] && vocab.has(s.slice(i - length, i))) {
        ok[i] = true;
        break;
      }
    }
  }
  return ok[s.length];
}
