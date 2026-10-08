function lexicalOrder(n: number): number[] {
  const result: number[] = [];
  let cur = 1;
  for (let i = 0; i < n; i++) {
    result.push(cur);
    if (cur * 10 <= n) {
      cur *= 10;
    } else {
      while (cur % 10 === 9 || cur + 1 > n) cur = Math.floor(cur / 10);
      cur++;
    }
  }
  return result;
}
