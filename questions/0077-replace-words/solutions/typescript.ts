type TrieNode = { children: Map<string, TrieNode>; end: boolean };

function replaceWords(roots: string[], sentence: string): string {
  const trie: TrieNode = { children: new Map(), end: false };
  for (const root of roots) {
    let node = trie;
    for (const ch of root) {
      let next = node.children.get(ch);
      if (next === undefined) {
        next = { children: new Map(), end: false };
        node.children.set(ch, next);
      }
      node = next;
    }
    node.end = true;
  }

  const shortestRoot = (word: string): string => {
    let node = trie;
    for (let i = 0; i < word.length; i++) {
      const next = node.children.get(word[i]);
      if (next === undefined) return word;
      if (next.end) return word.slice(0, i + 1);
      node = next;
    }
    return word;
  };

  return sentence.split(" ").map(shortestRoot).join(" ");
}
