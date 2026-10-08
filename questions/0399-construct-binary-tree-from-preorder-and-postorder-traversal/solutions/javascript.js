function constructFromPrePost(preorder, postorder) {
  const root = new TreeNode(preorder[0]);
  const stack = [root];
  let j = 0;
  for (let i = 1; i < preorder.length; i++) {
    const node = new TreeNode(preorder[i]);
    // Pop every node whose subtree is already complete.
    while (stack[stack.length - 1].val === postorder[j]) {
      stack.pop();
      j++;
    }
    const parent = stack[stack.length - 1];
    if (parent.left === null) parent.left = node;
    else parent.right = node;
    stack.push(node);
  }
  return root;
}
