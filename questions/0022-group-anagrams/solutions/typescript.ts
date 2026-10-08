function groupAnagrams(words: string[]): string[][] {
  const groups = new Map<string, string[]>();
  for (const w of words) {
    const counts = new Array<number>(26).fill(0);
    for (let i = 0; i < w.length; i++) counts[w.charCodeAt(i) - 97]++;
    const key = counts.join(",");
    const group = groups.get(key);
    if (group) group.push(w);
    else groups.set(key, [w]);
  }
  return [...groups.values()];
}
