function constructMaximumBinaryTree(nums) {
  const stack = [];
  for (const x of nums) {
    const node = new TreeNode(x);
    let last = null;
    while (stack.length > 0 && stack[stack.length - 1].val < x) {
      last = stack.pop();
    }
    node.left = last;
    if (stack.length > 0) stack[stack.length - 1].right = node;
    stack.push(node);
  }
  return stack[0];
}
