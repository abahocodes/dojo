function buildTree(preorder: number[], inorder: number[]): TreeNode | null {
    if (preorder.length === 0) return null;
    const root = new TreeNode(preorder[0]);
    const stack: TreeNode[] = [root];
    let j = 0; // next inorder position not yet closed off
    for (let i = 1; i < preorder.length; i++) {
        const val = preorder[i];
        let node = stack[stack.length - 1];
        if (node.val !== inorder[j]) {
            // node's left subtree is not finished, so val is its left child
            node.left = new TreeNode(val);
            stack.push(node.left);
        } else {
            // pop every node whose left side is complete; the last one popped
            // is the node whose right child val is
            while (stack.length > 0 && stack[stack.length - 1].val === inorder[j]) {
                node = stack.pop()!;
                j++;
            }
            node.right = new TreeNode(val);
            stack.push(node.right);
        }
    }
    return root;
}
