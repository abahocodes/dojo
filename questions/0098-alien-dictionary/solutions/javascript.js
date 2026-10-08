function alienOrder(words) {
  const letters = [...new Set(words.join(""))].sort();
  const after = new Map(letters.map((ch) => [ch, new Set()]));
  const indegree = new Map(letters.map((ch) => [ch, 0]));
  for (let i = 0; i + 1 < words.length; i++) {
    const first = words[i];
    const second = words[i + 1];
    const len = Math.min(first.length, second.length);
    let j = 0;
    while (j < len && first[j] === second[j]) j++;
    if (j < len) {
      const a = first[j];
      const b = second[j];
      if (!after.get(a).has(b)) {
        after.get(a).add(b);
        indegree.set(b, indegree.get(b) + 1);
      }
    } else if (first.length > second.length) {
      return ""; // a longer word sits before its own prefix
    }
  }

  // At most 26 letters, so picking the smallest ready letter by a linear
  // scan of the alphabetically sorted list is as fast as a heap.
  const done = new Set();
  let order = "";
  for (;;) {
    const ch = letters.find((x) => !done.has(x) && indegree.get(x) === 0);
    if (ch === undefined) break;
    done.add(ch);
    order += ch;
    for (const nxt of after.get(ch)) indegree.set(nxt, indegree.get(nxt) - 1);
  }
  return order.length === letters.length ? order : "";
}
