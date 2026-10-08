function widthOfBinaryTree(root: TreeNode): number {
  let best = 0;
  let nodes: TreeNode[] = [root];
  let positions: number[] = [0];
  while (nodes.length > 0) {
    const base = positions[0];
    best = Math.max(best, positions[positions.length - 1] - base + 1);
    const nextNodes: TreeNode[] = [];
    const nextPositions: number[] = [];
    for (let i = 0; i < nodes.length; i++) {
      const node = nodes[i];
      const pos = positions[i] - base;
      if (node.left) {
        nextNodes.push(node.left);
        nextPositions.push(2 * pos);
      }
      if (node.right) {
        nextNodes.push(node.right);
        nextPositions.push(2 * pos + 1);
      }
    }
    nodes = nextNodes;
    positions = nextPositions;
  }
  return best;
}
