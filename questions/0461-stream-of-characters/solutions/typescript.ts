function streamChecker(words: string[], stream: string): boolean[] {
  // Trie of reversed words, stored as 26 children per node.
  const child: Int32Array[] = [new Int32Array(26)];
  const isEnd: boolean[] = [false];
  let longest = 0;
  for (const word of words) {
    longest = Math.max(longest, word.length);
    let node = 0;
    for (let i = word.length - 1; i >= 0; i--) {
      const c = word.charCodeAt(i) - 97;
      if (child[node][c] === 0) {
        child[node][c] = child.length;
        child.push(new Int32Array(26));
        isEnd.push(false);
      }
      node = child[node][c];
    }
    isEnd[node] = true;
  }

  const result: boolean[] = [];
  for (let i = 0; i < stream.length; i++) {
    let node = 0;
    let found = false;
    for (let j = i; j >= 0 && j > i - longest; j--) {
      node = child[node][stream.charCodeAt(j) - 97];
      if (node === 0) break;
      if (isEnd[node]) {
        found = true;
        break;
      }
    }
    result.push(found);
  }
  return result;
}
