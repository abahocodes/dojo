function topKFrequentWords(words: string[], k: number): string[] {
  const counts = new Map<string, number>();
  for (const w of words) counts.set(w, (counts.get(w) ?? 0) + 1);
  const ranked = [...counts.keys()].sort((a, b) => {
    const diff = counts.get(b)! - counts.get(a)!;
    if (diff !== 0) return diff;
    return a < b ? -1 : a > b ? 1 : 0;
  });
  return ranked.slice(0, k);
}
