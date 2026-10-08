function exist(board, word) {
  const rows = board.length;
  const cols = board[0].length;

  const onBoard = new Map();
  for (const row of board) for (const ch of row) onBoard.set(ch, (onBoard.get(ch) ?? 0) + 1);
  const inWord = new Map();
  for (const ch of word) inWord.set(ch, (inWord.get(ch) ?? 0) + 1);
  for (const [ch, k] of inWord) if ((onBoard.get(ch) ?? 0) < k) return false;

  // a path read backwards is still a path; start from the rarer end to prune sooner
  if ((onBoard.get(word[0]) ?? 0) > (onBoard.get(word[word.length - 1]) ?? 0)) {
    word = [...word].reverse().join("");
  }

  function dfs(r, c, i) {
    if (r < 0 || r >= rows || c < 0 || c >= cols || board[r][c] !== word[i]) return false;
    if (i === word.length - 1) return true;
    board[r][c] = "#"; // mark as used on the current path
    const found =
      dfs(r + 1, c, i + 1) || dfs(r - 1, c, i + 1) || dfs(r, c + 1, i + 1) || dfs(r, c - 1, i + 1);
    board[r][c] = word[i];
    return found;
  }

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (dfs(r, c, 0)) return true;
    }
  }
  return false;
}
