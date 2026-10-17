class TrieNode {
  constructor() {
    this.children = new Map();
    this.isEnd = false;
  }
}

class Trie {
  constructor() {
    this.root = new TrieNode();
  }

  insert(word) {
    let node = this.root;
    for (const ch of word) {
      if (!node.children.has(ch)) node.children.set(ch, new TrieNode());
      node = node.children.get(ch);
    }
    node.isEnd = true;
  }

  // The node spelled by s, or null if the path leaves the trie.
  walk(s) {
    let node = this.root;
    for (const ch of s) {
      node = node.children.get(ch);
      if (!node) return null;
    }
    return node;
  }

  search(word) {
    const node = this.walk(word);
    return node !== null && node.isEnd;
  }

  startsWith(prefix) {
    return this.walk(prefix) !== null;
  }
}

// dojo calls this to run the tests; you don't need to change it.
function runTrie(ops, words) {
  const trie = new Trie();
  const out = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "insert") trie.insert(words[i]);
    else if (ops[i] === "search") out.push(trie.search(words[i]));
    else out.push(trie.startsWith(words[i]));
  }
  return out;
}
