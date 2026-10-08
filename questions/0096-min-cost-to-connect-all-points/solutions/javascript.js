function minCostConnectPoints(points) {
  const n = points.length;
  const best = new Array(n).fill(Infinity); // cheapest known edge from the tree to each point
  const inTree = new Array(n).fill(false);
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
    const [ux, uy] = points[u];
    for (let v = 0; v < n; v++) {
      if (!inTree[v]) {
        const d = Math.abs(points[v][0] - ux) + Math.abs(points[v][1] - uy);
        if (d < best[v]) best[v] = d;
      }
    }
  }
  return total;
}
