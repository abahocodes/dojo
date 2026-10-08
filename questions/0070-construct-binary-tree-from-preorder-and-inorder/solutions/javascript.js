function buildTree(preorder, inorder) {
  if (preorder.length === 0) return null;
  const root = new TreeNode(preorder[0]);
  const stack = [root];
  let j = 0;
  for (let i = 1; i < preorder.length; i++) {
    const val = preorder[i];
    let node = stack[stack.length - 1];
    if (node.val !== inorder[j]) {
      node.left = new TreeNode(val);
      stack.push(node.left);
    } else {
      while (stack.length > 0 && stack[stack.length - 1].val === inorder[j]) {
        node = stack.pop();
        j += 1;
      }
      node.right = new TreeNode(val);
      stack.push(node.right);
    }
  }
  return root;
}
