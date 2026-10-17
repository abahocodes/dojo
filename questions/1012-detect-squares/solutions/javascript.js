class DetectSquares {
  constructor() {
    this.cnt = new Map(); // x * 1001 + y -> copies stored
    this.byX = new Map(); // x -> Map(y -> copies stored)
  }

  add(point) {
    const [x, y] = point;
    const key = x * 1001 + y;
    this.cnt.set(key, (this.cnt.get(key) || 0) + 1);
    if (!this.byX.has(x)) this.byX.set(x, new Map());
    const col = this.byX.get(x);
    col.set(y, (col.get(y) || 0) + 1);
  }

  count(point) {
    const [qx, qy] = point;
    const col = this.byX.get(qx);
    if (!col) return 0;
    const at = (x, y) => (x < 0 || x > 1000 ? 0 : this.cnt.get(x * 1001 + y) || 0);
    let total = 0;
    // Every point in the query's column fixes the side length.
    for (const [y, c] of col) {
      const side = y - qy;
      if (side === 0) continue;
      for (const px of [qx + side, qx - side]) {
        total += c * at(px, qy) * at(px, y);
      }
    }
    return total;
  }
}

// dojo calls this to run the tests; you don't need to change it.
function runDetectSquares(ops, points) {
  const ds = new DetectSquares();
  const out = [];
  for (let i = 0; i < ops.length; i++) {
    if (ops[i] === "add") ds.add(points[i]);
    else out.push(ds.count(points[i]));
  }
  return out;
}
