function replaceWords(roots, sentence) {
  const trie = new Map();
  for (const root of roots) {
    let node = trie;
    for (const ch of root) {
      if (!node.has(ch)) node.set(ch, new Map());
      node = node.get(ch);
    }
    node.set("$", true);
  }

  const shortestRoot = (word) => {
    let node = trie;
    for (let i = 0; i < word.length; i++) {
      node = node.get(word[i]);
      if (node === undefined) return word;
      if (node.has("$")) return word.slice(0, i + 1);
    }
    return word;
  };

  return sentence.split(" ").map(shortestRoot).join(" ");
}
