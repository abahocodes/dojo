function indexPairs(text: string, words: string[]): number[][] {
  // Trie as an array of child maps; ends[node] marks the end of a word.
  const children: Array<Map<string, number>> = [new Map()];
  const ends: boolean[] = [false];
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

  const result: number[][] = [];
  for (let i = 0; i < text.length; i++) {
    let node: number | undefined = 0;
    for (let j = i; j < text.length; j++) {
      node = children[node].get(text[j]);
      if (node === undefined) break;
      if (ends[node]) result.push([i, j]);
    }
  }
  return result;
}
