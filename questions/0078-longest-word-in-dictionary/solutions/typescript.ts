type TrieNode = { children: Map<string, TrieNode>; word: string | null };

function longestWord(words: string[]): string {
  const trie: TrieNode = { children: new Map(), word: null };
  for (const word of words) {
    let node = trie;
    for (const ch of word) {
      let next = node.children.get(ch);
      if (next === undefined) {
        next = { children: new Map(), word: null };
        node.children.set(ch, next);
      }
      node = next;
    }
    node.word = word;
  }

  let best = "";
  const stack: TrieNode[] = [trie];
  while (stack.length > 0) {
    const node = stack.pop()!;
    for (const child of node.children.values()) {
      if (child.word === null) continue; // only walk through prefixes that are words themselves
      const word = child.word;
      if (word.length > best.length || (word.length === best.length && word < best)) {
        best = word;
      }
      stack.push(child);
    }
  }
  return best;
}
