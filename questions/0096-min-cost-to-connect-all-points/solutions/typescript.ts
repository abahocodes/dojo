function minCostConnectPoints(points: number[][]): number {
  const n = points.length;
  const xs = points.map((p) => p[0]);
  const ys = points.map((p) => p[1]);
  const best: number[] = new Array(n).fill(Infinity); // cheapest known edge from the tree to each point
  const inTree: boolean[] = new Array(n).fill(false);
  best[0] = 0;
  let total = 0;
  for (let step = 0; step < n; step++) {
    let u = -1;
    let cost = Infinity;
    for (let v = 0; v < n; v++) {
      if (!inTree[v] && best[v] < cost) {
        u = v;
        cost = best[v];
      }
    }
    inTree[u] = true;
    total += cost;
    const ux = xs[u];
    const uy = ys[u];
    for (let v = 0; v < n; v++) {
      if (!inTree[v]) {
        const d = Math.abs(xs[v] - ux) + Math.abs(ys[v] - uy);
        if (d < best[v]) best[v] = d;
      }
    }
  }
  return total;
}
