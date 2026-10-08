function indexPairs(text, words) {
  // Trie as an array of child maps; ends[node] marks the end of a word.
  const children = [new Map()];
  const ends = [false];
  for (const word of words) {
    let node = 0;
    for (const ch of word) {
      let next = children[node].get(ch);
      if (next === undefined) {
        next = children.length;
        children[node].set(ch, next);
        children.push(new Map());
        ends.push(false);
      }
      node = next;
    }
    ends[node] = true;
  }

  const result = [];
  for (let i = 0; i < text.length; i++) {
    let node = 0;
    for (let j = i; j < text.length; j++) {
      node = children[node].get(text[j]);
      if (node === undefined) break;
      if (ends[node]) result.push([i, j]);
    }
  }
  return result;
}
