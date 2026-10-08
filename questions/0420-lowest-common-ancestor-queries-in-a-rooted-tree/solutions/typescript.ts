function lcaQueries(parent: number[], queries: number[][]): number[] {
  const n = parent.length;
  const children: number[][] = Array.from({ length: n }, () => []);
  let root = 0;
  for (let v = 0; v < n; v++) {
    if (parent[v] === -1) root = v;
    else children[parent[v]].push(v);
  }

  const depth: number[] = new Array(n).fill(0);
  const queue: number[] = [root];
  for (let head = 0; head < queue.length; head++) {
    const u = queue[head];
    for (const c of children[u]) {
      depth[c] = depth[u] + 1;
      queue.push(c);
    }
  }

  let log = 1;
  while ((1 << log) < n) log++;
  const up: number[][] = [parent.map((p) => (p === -1 ? root : p))];
  for (let k = 1; k < log; k++) {
    const prev = up[k - 1];
    const cur: number[] = new Array(n);
    for (let v = 0; v < n; v++) cur[v] = prev[prev[v]];
    up.push(cur);
  }

  const answers: number[] = [];
  for (const [a, b] of queries) {
    let u = a;
    let v = b;
    if (depth[u] < depth[v]) [u, v] = [v, u];
    let diff = depth[u] - depth[v];
    for (let k = 0; diff > 0; k++, diff >>= 1) {
      if (diff & 1) u = up[k][u];
    }
    if (u !== v) {
      for (let k = log - 1; k >= 0; k--) {
        if (up[k][u] !== up[k][v]) {
          u = up[k][u];
          v = up[k][v];
        }
      }
      u = up[0][u];
    }
    answers.push(u);
  }
  return answers;
}
