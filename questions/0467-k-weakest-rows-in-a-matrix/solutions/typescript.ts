function kWeakestRows(mat: number[][], k: number): number[] {
  // Rows are 1s then 0s: binary search for the first 0.
  const soldiers = (row: number[]): number => {
    let lo = 0;
    let hi = row.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (row[mid] === 1) lo = mid + 1;
      else hi = mid;
    }
    return lo;
  };

  const counts = mat.map(soldiers);
  const order = mat.map((_, i) => i);
  order.sort((a, b) => counts[a] - counts[b] || a - b);
  return order.slice(0, k);
}
