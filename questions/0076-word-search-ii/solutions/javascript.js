function findWords(board, words) {
  const root = new Map();
  for (const word of words) {
    let node = root;
    for (const ch of word) {
      if (!node.has(ch)) node.set(ch, new Map());
      node = node.get(ch);
    }
    node.set("$", word);
  }

  const rows = board.length;
  const cols = board[0].length;
  const found = [];
  const dirs = [[1, 0], [-1, 0], [0, 1], [0, -1]];

  const dfs = (r, c, parent) => {
    const ch = board[r][c];
    const node = parent.get(ch);
    if (node === undefined) return;
    if (node.has("$")) {
      found.push(node.get("$"));
      node.delete("$");
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
    if (node.size === 0) parent.delete(ch); // prune branches with nothing left to find
  };

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      dfs(r, c, root);
    }
  }
  return found;
}
