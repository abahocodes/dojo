class WordNode {
  constructor() {
    this.children = new Map();
    this.isEnd = false;
  }
}

class WordDictionary {
  constructor() {
    this.root = new WordNode();
  }

  addWord(word) {
    let node = this.root;
    for (const ch of word) {
      if (!node.children.has(ch)) node.children.set(ch, new WordNode());
      node = node.children.get(ch);
    }
    node.isEnd = true;
  }

  search(word) {
    const match = (node, i) => {
      if (i === word.length) return node.isEnd;
      const ch = word[i];
      if (ch !== ".") {
        const child = node.children.get(ch);
        return child !== undefined && match(child, i + 1);
      }
      // A dot can be any letter: try every child.
      for (const child of node.children.values()) {
        if (match(child, i + 1)) return true;
      }
      return false;
    };
    return match(this.root, 0);
  }
}

// dojo calls this to run the tests; you don't need to change it.
function runWordDictionary(ops, words) {
  const wd = new WordDictionary();
  const out = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "addWord") wd.addWord(words[i]);
    else out.push(wd.search(words[i]));
  }
  return out;
}
