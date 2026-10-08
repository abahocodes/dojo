function exist(board: string[][], word: string): boolean {
    const rows = board.length;
    const cols = board[0].length;
    const onBoard = new Map<string, number>();
    for (const row of board) for (const ch of row) onBoard.set(ch, (onBoard.get(ch) ?? 0) + 1);
    const needed = new Map<string, number>();
    for (const ch of word) needed.set(ch, (needed.get(ch) ?? 0) + 1);
    for (const [ch, k] of needed) if ((onBoard.get(ch) ?? 0) < k) return false;
    // a path read backwards is still a path; start from the rarer end to prune sooner
    let w = word;
    if ((onBoard.get(w[0]) ?? 0) > (onBoard.get(w[w.length - 1]) ?? 0)) w = [...w].reverse().join("");

    const dfs = (r: number, c: number, i: number): boolean => {
        if (r < 0 || r >= rows || c < 0 || c >= cols || board[r][c] !== w[i]) return false;
        if (i === w.length - 1) return true;
        board[r][c] = "#"; // mark as used on the current path
        const found = dfs(r + 1, c, i + 1) || dfs(r - 1, c, i + 1) || dfs(r, c + 1, i + 1) || dfs(r, c - 1, i + 1);
        board[r][c] = w[i];
        return found;
    };

    for (let r = 0; r < rows; r++) for (let c = 0; c < cols; c++) if (dfs(r, c, 0)) return true;
    return false;
}
