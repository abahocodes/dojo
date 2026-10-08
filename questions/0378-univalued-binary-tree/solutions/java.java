class Solution {
    public boolean isUnivalTree(TreeNode root) {
        int value = root.val;
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (node.val != value) return false;
            if (node.left != null) stack.push(node.left);
            if (node.right != null) stack.push(node.right);
        }
        return true;
    }
}
