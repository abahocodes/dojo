function wordSquares(words) {
  const size = words[0].length;
  // Every prefix (including the empty one) -> the words starting with it.
  const byPrefix = new Map();
  for (const word of words) {
    for (let i = 0; i <= size; i++) {
      const p = word.slice(0, i);
      if (!byPrefix.has(p)) byPrefix.set(p, []);
      byPrefix.get(p).push(word);
    }
  }

  const result = [];
  const square = [];
  const backtrack = () => {
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
