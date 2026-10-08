function findDuplicateSubtrees(root: TreeNode | null): Array<TreeNode | null> {
  const ids = new Map<string, number>();      // "left id,value,right id" -> subtree id
  const count = new Map<number, number>();    // subtree id -> occurrences
  const nodeId = new Map<TreeNode, number>(); // node -> subtree id (0 means empty)
  const result: TreeNode[] = [];
  const idOf = (node: TreeNode | null): number => (node === null ? 0 : nodeId.get(node)!);
  const stack: Array<[TreeNode | null, boolean]> = [[root, false]];
  while (stack.length > 0) {
    const [node, done] = stack.pop()!;
    if (node === null) continue;
    if (!done) {
      stack.push([node, true], [node.right, false], [node.left, false]);
      continue;
    }
    const key = `${idOf(node.left)},${node.val},${idOf(node.right)}`;
    if (!ids.has(key)) ids.set(key, ids.size + 1);
    const sid = ids.get(key)!;
    nodeId.set(node, sid);
    const seen = (count.get(sid) ?? 0) + 1;
    count.set(sid, seen);
    if (seen === 2) result.push(node);
  }
  return result;
}
