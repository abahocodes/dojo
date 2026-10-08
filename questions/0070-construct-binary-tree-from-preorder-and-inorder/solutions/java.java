class Solution {
    public TreeNode buildTree(int[] preorder, int[] inorder) {
        if (preorder.length == 0) return null;
        TreeNode root = new TreeNode(preorder[0]);
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        int j = 0; // next inorder position not yet closed off
        for (int i = 1; i < preorder.length; i++) {
            int val = preorder[i];
            TreeNode node = stack.peek();
            if (node.val != inorder[j]) {
                // node's left subtree is not finished, so val is its left child
                node.left = new TreeNode(val);
                stack.push(node.left);
            } else {
                // pop every node whose left side is complete; the last one popped
                // is the node whose right child val is
                while (!stack.isEmpty() && stack.peek().val == inorder[j]) {
                    node = stack.pop();
                    j++;
                }
                node.right = new TreeNode(val);
                stack.push(node.right);
            }
        }
        return root;
    }
}
