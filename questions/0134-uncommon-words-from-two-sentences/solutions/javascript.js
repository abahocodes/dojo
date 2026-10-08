function uncommonFromSentences(s1, s2) {
  const counts = new Map();
  for (const w of (s1 + " " + s2).split(" ")) {
    counts.set(w, (counts.get(w) || 0) + 1);
  }
  const result = [];
  for (const [w, c] of counts) if (c === 1) result.push(w);
  return result;
}
