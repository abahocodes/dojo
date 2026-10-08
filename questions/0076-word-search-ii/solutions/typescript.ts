type TrieNode = { children: Map<string, TrieNode>; word: string | null };

function findWords(board: string[][], words: string[]): string[] {
  const root: TrieNode = { children: new Map(), word: null };
  for (const word of words) {
    let node = root;
    for (const ch of word) {
      let next = node.children.get(ch);
      if (next === undefined) {
        next = { children: new Map(), word: null };
        node.children.set(ch, next);
      }
      node = next;
    }
    node.word = word;
  }

  const rows = board.length;
  const cols = board[0].length;
  const found: string[] = [];
  const dirs: [number, number][] = [[1, 0], [-1, 0], [0, 1], [0, -1]];

  const dfs = (r: number, c: number, parent: TrieNode): void => {
    const ch = board[r][c];
    const node = parent.children.get(ch);
    if (node === undefined) return;
    if (node.word !== null) {
      found.push(node.word);
      node.word = null;
    }
    board[r][c] = "#";
    for (const [dr, dc] of dirs) {
      const nr = r + dr;
      const nc = c + dc;
      if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && board[nr][nc] !== "#") {
        dfs(nr, nc, node);
      }
    }
    board[r][c] = ch;
    // prune branches with nothing left to find
    if (node.children.size === 0 && node.word === null) parent.children.delete(ch);
  };

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      dfs(r, c, root);
    }
  }
  return found;
}
