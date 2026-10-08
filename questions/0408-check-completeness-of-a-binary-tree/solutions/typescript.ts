function isCompleteTree(root: TreeNode | null): boolean {
  const queue: (TreeNode | null)[] = [root];
  let seenGap = false;
  for (let head = 0; head < queue.length; head++) {
    const node = queue[head];
    if (node === null) {
      seenGap = true;
      continue;
    }
    if (seenGap) return false;
    queue.push(node.left, node.right);
  }
  return true;
}
