function maximizeXor(nums: number[], queries: number[][]): number[] {
  const BITS = 30;
  const sorted = [...nums].sort((a, b) => a - b);
  const order = queries.map((_, i) => i).sort((a, b) => queries[a][1] - queries[b][1]);
  const maxNodes = sorted.length * BITS + 1;
  const child = new Int32Array(maxNodes * 2); // child[2 * node + bit]
  let nodes = 1;
  const answer: number[] = new Array(queries.length).fill(-1);
  let j = 0;

  for (const qi of order) {
    const [x, limit] = queries[qi];
    while (j < sorted.length && sorted[j] <= limit) {
      const v = sorted[j];
      let node = 0;
      for (let b = BITS - 1; b >= 0; b--) {
        const bit = (v >> b) & 1;
        if (child[2 * node + bit] === 0) child[2 * node + bit] = nodes++;
        node = child[2 * node + bit];
      }
      j++;
    }
    if (j === 0) continue;
    let node = 0;
    let best = 0;
    for (let b = BITS - 1; b >= 0; b--) {
      const want = ((x >> b) & 1) ^ 1;
      if (child[2 * node + want] !== 0) {
        best |= 1 << b;
        node = child[2 * node + want];
      } else {
        node = child[2 * node + (want ^ 1)];
      }
    }
    answer[qi] = best;
  }

  return answer;
}
