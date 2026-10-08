function distanceK(root: TreeNode | null, target: number, k: number): number[] {
  // Turn the tree into an undirected graph keyed by value.
  const adj = new Map<number, number[]>([[root!.val, []]]);
  const stack: TreeNode[] = [root!];
  while (stack.length > 0) {
    const node = stack.pop()!;
    for (const child of [node.left, node.right]) {
      if (child !== null) {
        adj.get(node.val)!.push(child.val);
        adj.set(child.val, [node.val]);
        stack.push(child);
      }
    }
  }
  let frontier: number[] = [target];
  const seen = new Set<number>([target]);
  for (let step = 0; step < k && frontier.length > 0; step++) {
    const next: number[] = [];
    for (const v of frontier) {
      for (const w of adj.get(v)!) {
        if (!seen.has(w)) {
          seen.add(w);
          next.push(w);
        }
      }
    }
    frontier = next;
  }
  return frontier;
}
