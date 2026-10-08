function longestWord(words) {
  const trie = new Map();
  for (const word of words) {
    let node = trie;
    for (const ch of word) {
      if (!node.has(ch)) node.set(ch, new Map());
      node = node.get(ch);
    }
    node.set("$", word);
  }

  let best = "";
  const stack = [trie];
  while (stack.length > 0) {
    const node = stack.pop();
    for (const [ch, child] of node) {
      if (ch === "$" || !child.has("$")) continue; // only walk through prefixes that are words themselves
      const word = child.get("$");
      if (word.length > best.length || (word.length === best.length && word < best)) {
        best = word;
      }
      stack.push(child);
    }
  }
  return best;
}
