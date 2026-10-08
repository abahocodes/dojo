class Solution {
    public TreeNode bstFromPreorder(int[] preorder) {
        TreeNode root = new TreeNode(preorder[0]);
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        for (int i = 1; i < preorder.length; i++) {
            int v = preorder[i];
            TreeNode node = new TreeNode(v);
            if (v < stack.peek().val) {
                stack.peek().left = node;
            } else {
                TreeNode parent = stack.pop();
                while (!stack.isEmpty() && stack.peek().val < v) parent = stack.pop();
                parent.right = node;
            }
            stack.push(node);
        }
        return root;
    }
}
