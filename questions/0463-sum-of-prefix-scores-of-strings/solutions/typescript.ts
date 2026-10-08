function sumPrefixScores(words: string[]): number[] {
  // Trie where every node counts how many words pass through it.
  const child: Int32Array[] = [new Int32Array(26)];
  const count: number[] = [0];
  for (const word of words) {
    let node = 0;
    for (let i = 0; i < word.length; i++) {
      const c = word.charCodeAt(i) - 97;
      if (child[node][c] === 0) {
        child[node][c] = child.length;
        child.push(new Int32Array(26));
        count.push(0);
      }
      node = child[node][c];
      count[node]++;
    }
  }

  return words.map((word) => {
    let node = 0;
    let total = 0;
    for (let i = 0; i < word.length; i++) {
      node = child[node][word.charCodeAt(i) - 97];
      total += count[node];
    }
    return total;
  });
}
