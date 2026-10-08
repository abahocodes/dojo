function deepestLeavesSum(root) {
  let level = [root];
  while (true) {
    const next = [];
    for (const node of level) {
      if (node.left) next.push(node.left);
      if (node.right) next.push(node.right);
    }
    if (next.length === 0) {
      let sum = 0;
      for (const node of level) sum += node.val;
      return sum;
    }
    level = next;
  }
}
