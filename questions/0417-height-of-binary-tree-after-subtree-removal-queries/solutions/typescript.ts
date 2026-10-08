function treeQueries(root: TreeNode | null, queries: number[]): number[] {
  const order: TreeNode[] = [];
  const depthOf: number[] = [];
  const stack: TreeNode[] = root ? [root] : [];
  const depths: number[] = [0];
  while (stack.length > 0) {
    const node = stack.pop()!;
    const d = depths.pop()!;
    order.push(node);
    depthOf.push(d);
    if (node.left) {
      stack.push(node.left);
      depths.push(d + 1);
    }
    if (node.right) {
      stack.push(node.right);
      depths.push(d + 1);
    }
  }
  const n = order.length;
  const depth = new Int32Array(n + 1);
  const height = new Int32Array(n + 1);
  let levels = 0;
  for (let i = n - 1; i >= 0; i--) {
    const node = order[i];
    depth[node.val] = depthOf[i];
    levels = Math.max(levels, depthOf[i] + 1);
    let h = 0;
    if (node.left) h = height[node.left.val] + 1;
    if (node.right) h = Math.max(h, height[node.right.val] + 1);
    height[node.val] = h;
  }
  const best1 = new Int32Array(levels).fill(-1);
  const best2 = new Int32Array(levels).fill(-1);
  const owner = new Int32Array(levels);
  for (let v = 1; v <= n; v++) {
    const d = depth[v];
    const reach = d + height[v];
    if (reach > best1[d]) {
      best2[d] = best1[d];
      best1[d] = reach;
      owner[d] = v;
    } else if (reach > best2[d]) {
      best2[d] = reach;
    }
  }
  return queries.map((q) => {
    const d = depth[q];
    if (owner[d] !== q) return best1[d];
    if (best2[d] >= 0) return best2[d];
    return d - 1;
  });
}
