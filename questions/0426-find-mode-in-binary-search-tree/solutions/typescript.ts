function findMode(root: TreeNode | null): number[] {
  let modes: number[] = [];
  let best = 0;
  let count = 0;
  let prev: number | null = null;
  const stack: TreeNode[] = [];
  let node = root;
  while (stack.length > 0 || node) {
    while (node) {
      stack.push(node);
      node = node.left;
    }
    const cur = stack.pop()!;
    count = cur.val === prev ? count + 1 : 1;
    prev = cur.val;
    if (count > best) {
      best = count;
      modes = [cur.val];
    } else if (count === best) {
      modes.push(cur.val);
    }
    node = cur.right;
  }
  return modes;
}
