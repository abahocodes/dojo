function createBinaryTree(descriptions) {
  const nodes = new Map();
  const children = new Set();
  const get = (value) => {
    if (!nodes.has(value)) nodes.set(value, new TreeNode(value));
    return nodes.get(value);
  };
  for (const [parent, child, isLeft] of descriptions) {
    if (isLeft === 1) get(parent).left = get(child);
    else get(parent).right = get(child);
    children.add(child);
  }
  for (const [parent] of descriptions) {
    if (!children.has(parent)) return nodes.get(parent);
  }
  return null;
}
