function shortestUniquePrefixes(words: string[]): string[] {
  const children: Int32Array[] = [new Int32Array(26)];
  const count: number[] = [0];
  for (const w of words) {
    let node = 0;
    for (let i = 0; i < w.length; i++) {
      const c = w.charCodeAt(i) - 97;
      if (children[node][c] === 0) {
        children[node][c] = children.length;
        children.push(new Int32Array(26));
        count.push(0);
      }
      node = children[node][c];
      count[node]++;
    }
  }
  const result: string[] = [];
  for (const w of words) {
    let node = 0;
    let length = w.length;
    for (let i = 0; i < w.length; i++) {
      node = children[node][w.charCodeAt(i) - 97];
      if (count[node] === 1) {
        length = i + 1;
        break;
      }
    }
    result.push(w.slice(0, length));
  }
  return result;
}
