class Solution {
    public TreeNode buildTreeInPost(int[] inorder, int[] postorder) {
        int n = postorder.length;
        if (n == 0) return null;
        // Walk postorder backwards (root, right, left) and inorder backwards.
        TreeNode root = new TreeNode(postorder[n - 1]);
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        int i = n - 1;
        for (int j = n - 2; j >= 0; j--) {
            TreeNode node = new TreeNode(postorder[j]);
            TreeNode parent = stack.peek();
            if (parent.val != inorder[i]) {
                parent.right = node;
            } else {
                while (!stack.isEmpty() && stack.peek().val == inorder[i]) {
                    parent = stack.pop();
                    i--;
                }
                parent.left = node;
            }
            stack.push(node);
        }
        return root;
    }
}
