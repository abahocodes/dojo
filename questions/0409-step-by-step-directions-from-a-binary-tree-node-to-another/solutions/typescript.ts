function getDirections(root: TreeNode | null, startValue: number, destValue: number): string {
  if (root === null) return "";
  const parent = new Map<number, number | null>([[root.val, null]]);
  const move = new Map<number, string>();
  const stack: TreeNode[] = [root];
  while (stack.length > 0) {
    const node = stack.pop()!;
    if (node.left !== null) {
      parent.set(node.left.val, node.val);
      move.set(node.left.val, "L");
      stack.push(node.left);
    }
    if (node.right !== null) {
      parent.set(node.right.val, node.val);
      move.set(node.right.val, "R");
      stack.push(node.right);
    }
  }
  const pathFromRoot = (value: number): string[] => {
    const moves: string[] = [];
    let cur = value;
    let up = parent.get(cur);
    while (up !== null && up !== undefined) {
      moves.push(move.get(cur)!);
      cur = up;
      up = parent.get(cur);
    }
    return moves.reverse();
  };
  const toStart = pathFromRoot(startValue);
  const toDest = pathFromRoot(destValue);
  let common = 0;
  while (common < toStart.length && common < toDest.length && toStart[common] === toDest[common]) {
    common++;
  }
  return "U".repeat(toStart.length - common) + toDest.slice(common).join("");
}
