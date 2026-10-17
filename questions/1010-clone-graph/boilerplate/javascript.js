// 1010. Clone Graph (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Node {
  constructor(val = 0, neighbors = []) {
    this.val = val;
    this.neighbors = neighbors;
  }
}

/**
 * @param {Node | null} node
 * @return {Node | null}
 */
function cloneGraph(node) {

}

// dojo calls this to run the tests; you don't need to change it.
function runCloneGraph(adjList) {
  const n = adjList.length;
  const originals = adjList.map((_, i) => new Node(i + 1));
  adjList.forEach((nbrs, i) => {
    originals[i].neighbors = nbrs.map((v) => originals[v - 1]);
  });
  const clone = cloneGraph(n > 0 ? originals[0] : null);

  // The original graph must be left as it was.
  for (let i = 0; i < n; i++) {
    const node = originals[i];
    const nbrs = adjList[i];
    if (node.val !== i + 1 || node.neighbors.length !== nbrs.length ||
        nbrs.some((v, j) => node.neighbors[j] !== originals[v - 1])) {
      return [[-1]];
    }
  }
  if (clone == null) return [];
  // Explore the clone: no original node, one node per value 1..n.
  const originalSet = new Set(originals);
  const seen = new Set();
  const byVal = new Map();
  const stack = [clone];
  while (stack.length > 0) {
    const node = stack.pop();
    if (originalSet.has(node)) return [[-1]];
    if (seen.has(node)) continue;
    seen.add(node);
    if (byVal.has(node.val) || !(node.val >= 1 && node.val <= n)) return [[-1]];
    byVal.set(node.val, node);
    stack.push(...node.neighbors);
  }
  const result = [];
  for (let v = 1; v <= n; v++) {
    const node = byVal.get(v);
    result.push(node ? node.neighbors.map((nb) => nb.val).sort((a, b) => a - b) : []);
  }
  return result;
}
