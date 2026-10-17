// 1012. Detect Squares (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class DetectSquares {
  constructor() {

  }

  /** @param {number[]} point */
  add(point) {

  }

  /** @param {number[]} point @return {number} */
  count(point) {
    return 0;
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
