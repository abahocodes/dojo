function sumEvenGrandparent(root: TreeNode): number {
  let total = 0;
  const stack: TreeNode[] = [root];
  while (stack.length > 0) {
    const node = stack.pop()!;
    for (const child of [node.left, node.right]) {
      if (child === null) continue;
      if (node.val % 2 === 0) {
        if (child.left) total += child.left.val;
        if (child.right) total += child.right.val;
      }
      stack.push(child);
    }
  }
  return total;
}
