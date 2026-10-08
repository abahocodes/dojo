class Solution {
    public TreeNode constructFromPrePost(int[] preorder, int[] postorder) {
        TreeNode root = new TreeNode(preorder[0]);
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        int j = 0;
        for (int i = 1; i < preorder.length; i++) {
            TreeNode node = new TreeNode(preorder[i]);
            // Pop every node whose subtree is already complete.
            while (stack.peek().val == postorder[j]) {
                stack.pop();
                j++;
            }
            TreeNode parent = stack.peek();
            if (parent.left == null) parent.left = node;
            else parent.right = node;
            stack.push(node);
        }
        return root;
    }
}
