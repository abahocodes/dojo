function solveNQueens(n: number): string[][] {
    const cols = new Set<number>();
    const diag = new Set<number>();
    const anti = new Set<number>();
    const queenCol: number[] = new Array(n).fill(0); // queenCol[r] = column of the queen in row r
    const boards: string[][] = [];

    const place = (r: number): void => {
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
    };

    place(0);
    return boards;
}
