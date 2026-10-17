function solveNQueens(n) {
  const cols = new Set();
  const diag = new Set();
  const anti = new Set();
  const queenCol = new Array(n).fill(0); // queenCol[r] = column of the queen in row r
  const boards = [];

  function place(r) {
    if (r === n) {
      boards.push(queenCol.map((c) => ".".repeat(c) + "Q" + ".".repeat(n - 1 - c)));
      return;
    }
    for (let c = 0; c < n; c++) {
      if (cols.has(c) || diag.has(r - c) || anti.has(r + c)) continue;
      queenCol[r] = c;
      cols.add(c); diag.add(r - c); anti.add(r + c);
      place(r + 1);
      cols.delete(c); diag.delete(r - c); anti.delete(r + c);
    }
  }

  place(0);
  return boards;
}
