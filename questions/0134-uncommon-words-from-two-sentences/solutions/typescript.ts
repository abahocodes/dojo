function uncommonFromSentences(s1: string, s2: string): string[] {
  const counts = new Map<string, number>();
  for (const w of (s1 + " " + s2).split(" ")) {
    counts.set(w, (counts.get(w) ?? 0) + 1);
  }
  const result: string[] = [];
  for (const [w, c] of counts) if (c === 1) result.push(w);
  return result;
}
