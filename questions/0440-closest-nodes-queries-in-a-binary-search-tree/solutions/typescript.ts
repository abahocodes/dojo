function closestNodes(root: TreeNode | null, queries: number[]): number[][] {
  const values: number[] = [];
  const stack: TreeNode[] = [];
  let node: TreeNode | null = root;
  while (stack.length > 0 || node !== null) {
    while (node !== null) {
      stack.push(node);
      node = node.left;
    }
    const top = stack.pop()!;
    values.push(top.val);
    node = top.right;
  }
  const answer: number[][] = [];
  for (const q of queries) {
    let lo = 0;
    let hi = values.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (values[mid] < q) lo = mid + 1;
      else hi = mid;
    }
    if (lo < values.length && values[lo] === q) {
      answer.push([q, q]);
    } else {
      answer.push([lo > 0 ? values[lo - 1] : -1, lo < values.length ? values[lo] : -1]);
    }
  }
  return answer;
}
