function distanceK(root, target, k) {
  // Turn the tree into an undirected graph keyed by value.
  const adj = new Map([[root.val, []]]);
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    for (const child of [node.left, node.right]) {
      if (child !== null) {
        adj.get(node.val).push(child.val);
        adj.set(child.val, [node.val]);
        stack.push(child);
      }
    }
  }
  let frontier = [target];
  const seen = new Set([target]);
  for (let step = 0; step < k && frontier.length > 0; step++) {
    const next = [];
    for (const v of frontier) {
      for (const w of adj.get(v)) {
        if (!seen.has(w)) {
          seen.add(w);
          next.push(w);
        }
      }
    }
    frontier = next;
  }
  return frontier;
}
