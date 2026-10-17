// 1007. Implement Trie (Prefix Tree) (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Trie {
  constructor() {

  }

  /** @param {string} word */
  insert(word) {

  }

  /** @param {string} word @return {boolean} */
  search(word) {
    return false;
  }

  /** @param {string} prefix @return {boolean} */
  startsWith(prefix) {
    return false;
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
