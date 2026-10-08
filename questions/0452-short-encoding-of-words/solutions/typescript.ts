function minimumLengthEncoding(words: string[]): number {
  const keep = new Set<string>(words);
  for (const w of new Set<string>(words)) {
    for (let k = 1; k < w.length; k++) keep.delete(w.slice(k));
  }
  let total = 0;
  for (const w of keep) total += w.length + 1;
  return total;
}
