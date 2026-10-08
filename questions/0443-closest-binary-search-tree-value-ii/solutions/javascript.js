function closestKValues(root, target, k) {
  // pred: path to values <= target (top = largest); succ: values > target (top = smallest).
  const pred = [];
  const succ = [];
  let node = root;
  while (node !== null) {
    if (node.val <= target) {
      pred.push(node);
      node = node.right;
    } else {
      succ.push(node);
      node = node.left;
    }
  }
  const nextSmaller = () => {
    const top = pred.pop();
    for (let cur = top.left; cur !== null; cur = cur.right) pred.push(cur);
    return top.val;
  };
  const nextLarger = () => {
    const top = succ.pop();
    for (let cur = top.right; cur !== null; cur = cur.left) succ.push(cur);
    return top.val;
  };
  const result = [];
  for (let i = 0; i < k; i++) {
    if (succ.length === 0 ||
        (pred.length > 0 && target - pred[pred.length - 1].val <= succ[succ.length - 1].val - target)) {
      result.push(nextSmaller());
    } else {
      result.push(nextLarger());
    }
  }
  return result;
}
