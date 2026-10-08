function groupAnagrams(words) {
  const groups = new Map();
  for (const w of words) {
    const counts = new Array(26).fill(0);
    for (let i = 0; i < w.length; i++) counts[w.charCodeAt(i) - 97]++;
    const key = counts.join("#");
    if (!groups.has(key)) groups.set(key, []);
    groups.get(key).push(w);
  }
  return [...groups.values()];
}
