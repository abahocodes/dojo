function findKthNumber(m, n, k) {
  if (m > n) [m, n] = [n, m];
  const countAtMost = (x) => {
    let total = 0;
    for (let i = 1; i <= m; i++) {
      const inRow = Math.floor(x / i);
      if (inRow === 0) break;
      total += Math.min(inRow, n);
    }
    return total;
  };

  let lo = 1;
  let hi = m * n;
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (countAtMost(mid) >= k) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}
