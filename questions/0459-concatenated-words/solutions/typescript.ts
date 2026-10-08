function findAllConcatenatedWords(words: string[]): string[] {
  const order = words.map((_, i) => i).sort((a, b) => words[a].length - words[b].length);
  const known = new Set<string>();
  const found: boolean[] = new Array(words.length).fill(false);
  for (const i of order) {
    const w = words[i];
    if (known.size > 0) {
      const n = w.length;
      const can: boolean[] = new Array(n + 1).fill(false);
      can[0] = true;
      for (let end = 1; end <= n; end++) {
        for (let start = 0; start < end; start++) {
          if (can[start] && known.has(w.slice(start, end))) {
            can[end] = true;
            break;
          }
        }
      }
      found[i] = can[n];
    }
    known.add(w);
  }
  return words.filter((_, i) => found[i]);
}
