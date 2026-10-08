function createBinaryTree(descriptions: number[][]): TreeNode | null {
  const nodes = new Map<number, TreeNode>();
  const children = new Set<number>();
  const get = (value: number): TreeNode => {
    let node = nodes.get(value);
    if (node === undefined) {
      node = new TreeNode(value);
      nodes.set(value, node);
    }
    return node;
  };
  for (const [parent, child, isLeft] of descriptions) {
    if (isLeft === 1) get(parent).left = get(child);
    else get(parent).right = get(child);
    children.add(child);
  }
  for (const [parent] of descriptions) {
    if (!children.has(parent)) return nodes.get(parent)!;
  }
  return null;
}
