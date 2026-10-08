function amountOfTime(root, start) {
  const parent = new Map([[root, null]]);
  let source = null;
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
    if (node.val === start) source = node;
    for (const child of [node.left, node.right]) {
      if (child !== null) {
        parent.set(child, node);
        stack.push(child);
      }
    }
  }
  const seen = new Set([source]);
  let frontier = [source];
  let minutes = -1;
  while (frontier.length > 0) {
    minutes++;
    const next = [];
    for (const node of frontier) {
      for (const neighbor of [node.left, node.right, parent.get(node)]) {
        if (neighbor !== null && !seen.has(neighbor)) {
          seen.add(neighbor);
          next.push(neighbor);
        }
      }
    }
    frontier = next;
  }
  return minutes;
}
