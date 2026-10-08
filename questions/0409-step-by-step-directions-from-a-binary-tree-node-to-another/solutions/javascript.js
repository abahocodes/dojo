function getDirections(root, startValue, destValue) {
  const parent = new Map([[root.val, null]]);
  const move = new Map();
  const stack = [root];
  while (stack.length > 0) {
    const node = stack.pop();
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
  const pathFromRoot = (value) => {
    const moves = [];
    while (parent.get(value) !== null) {
      moves.push(move.get(value));
      value = parent.get(value);
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
