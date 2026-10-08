function stringIndices(wordsContainer, wordsQuery) {
  let total = 0;
  for (const w of wordsContainer) total += w.length;
  const children = new Int32Array((total + 1) * 26);
  const best = new Int32Array(total + 1);
  let nodes = 1;
  for (let i = 0; i < wordsContainer.length; i++) {
    const w = wordsContainer[i];
    if (w.length < wordsContainer[best[0]].length) best[0] = i;
    let node = 0;
    for (let k = w.length - 1; k >= 0; k--) {
      const slot = node * 26 + (w.charCodeAt(k) - 97);
      if (children[slot] === 0) {
        children[slot] = nodes;
        best[nodes] = i;
        nodes++;
      } else if (w.length < wordsContainer[best[children[slot]]].length) {
        best[children[slot]] = i;
      }
      node = children[slot];
    }
  }
  const result = [];
  for (const q of wordsQuery) {
    let node = 0;
    for (let k = q.length - 1; k >= 0; k--) {
      const next = children[node * 26 + (q.charCodeAt(k) - 97)];
      if (next === 0) break;
      node = next;
    }
    result.push(best[node]);
  }
  return result;
}
