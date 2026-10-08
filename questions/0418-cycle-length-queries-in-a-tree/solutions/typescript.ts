function cycleLengthQueries(n: number, queries: number[][]): number[] {
  return queries.map(([a, b]) => {
    let steps = 0;
    while (a !== b) {
      if (a > b) a >>= 1;
      else b >>= 1;
      steps++;
    }
    return steps + 1;
  });
}
