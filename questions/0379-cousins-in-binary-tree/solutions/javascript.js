function isCousins(root, x, y) {
  let level = [root];
  while (level.length > 0) {
    let parentX = null;
    let parentY = null;
    const next = [];
    for (const node of level) {
      for (const child of [node.left, node.right]) {
        if (child === null) continue;
        if (child.val === x) parentX = node;
        else if (child.val === y) parentY = node;
        next.push(child);
      }
    }
    if (parentX !== null && parentY !== null) return parentX !== parentY;
    if (parentX !== null || parentY !== null) return false;
    level = next;
  }
  return false;
}
