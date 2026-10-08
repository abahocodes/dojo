function alienOrder(words: string[]): string {
  const A = "a".charCodeAt(0);
  const present: boolean[] = new Array(26).fill(false);
  for (const w of words) for (let i = 0; i < w.length; i++) present[w.charCodeAt(i) - A] = true;
  const after: Set<number>[] = Array.from({ length: 26 }, () => new Set<number>());
  const indegree: number[] = new Array(26).fill(0);
  for (let k = 0; k + 1 < words.length; k++) {
    const first = words[k];
    const second = words[k + 1];
    const len = Math.min(first.length, second.length);
    let differs = false;
    for (let i = 0; i < len; i++) {
      const a = first.charCodeAt(i) - A;
      const b = second.charCodeAt(i) - A;
      if (a !== b) {
        if (!after[a].has(b)) {
          after[a].add(b);
          indegree[b]++;
        }
        differs = true;
        break;
      }
    }
    if (!differs && first.length > second.length) return ""; // a longer word sits before its own prefix
  }
  // Only 26 letters, so picking the smallest ready letter by scanning stands in for a heap.
  const ready: boolean[] = new Array(26).fill(false);
  let total = 0;
  for (let ch = 0; ch < 26; ch++) {
    if (present[ch]) {
      total++;
      if (indegree[ch] === 0) ready[ch] = true;
    }
  }
  let order = "";
  for (;;) {
    const ch = ready.indexOf(true);
    if (ch < 0) break;
    ready[ch] = false;
    order += String.fromCharCode(A + ch);
    for (const nxt of after[ch]) {
      if (--indegree[nxt] === 0) ready[nxt] = true;
    }
  }
  return order.length === total ? order : "";
}
