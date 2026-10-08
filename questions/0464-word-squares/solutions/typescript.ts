function wordSquares(words: string[]): string[][] {
  const size = words[0].length;
  // Every prefix (including the empty one) -> the words starting with it.
  const byPrefix = new Map<string, string[]>();
  for (const word of words) {
    for (let i = 0; i <= size; i++) {
      const p = word.slice(0, i);
      let list = byPrefix.get(p);
      if (list === undefined) {
        list = [];
        byPrefix.set(p, list);
      }
      list.push(word);
    }
  }

  const result: string[][] = [];
  const square: string[] = [];
  const backtrack = (): void => {
    const k = square.length;
    if (k === size) {
      result.push([...square]);
      return;
    }
    // Row k must start with column k of the rows placed so far.
    let prefix = '';
    for (const row of square) prefix += row[k];
    for (const word of byPrefix.get(prefix) ?? []) {
      square.push(word);
      backtrack();
      square.pop();
    }
  };

  backtrack();
  return result;
}
