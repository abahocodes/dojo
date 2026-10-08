function buildTreeInPost(inorder, postorder) {
  const n = postorder.length;
  if (n === 0) return null;
  // Walk postorder backwards (root, right, left) and inorder backwards.
  const root = new TreeNode(postorder[n - 1]);
  const stack = [root];
  let i = n - 1;
  for (let j = n - 2; j >= 0; j--) {
    const node = new TreeNode(postorder[j]);
    let parent = stack[stack.length - 1];
    if (parent.val !== inorder[i]) {
      parent.right = node;
    } else {
      while (stack.length > 0 && stack[stack.length - 1].val === inorder[i]) {
        parent = stack.pop();
        i--;
      }
      parent.left = node;
    }
    stack.push(node);
  }
  return root;
}
