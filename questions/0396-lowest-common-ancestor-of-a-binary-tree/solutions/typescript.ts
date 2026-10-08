function lowestCommonAncestor(root: TreeNode | null, p: number, q: number): number {
  const parent = new Map<number, number | null>([[root!.val, null]]);
  const stack: TreeNode[] = [root!];
  while (stack.length > 0 && !(parent.has(p) && parent.has(q))) {
    const node = stack.pop()!;
    for (const child of [node.left, node.right]) {
      if (child !== null) {
        parent.set(child.val, node.val);
        stack.push(child);
      }
    }
  }
  const ancestors = new Set<number>();
  for (let v: number | null = p; v !== null; v = parent.get(v)!) ancestors.add(v);
  let v = q;
  while (!ancestors.has(v)) v = parent.get(v)!;
  return v;
}
