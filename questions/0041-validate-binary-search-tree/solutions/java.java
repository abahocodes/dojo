class Solution {
    public boolean isValidBst(TreeNode root) {
        Deque<TreeNode> stack = new ArrayDeque<>();
        Integer prev = null;
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            if (prev != null && node.val <= prev) return false;
            prev = node.val;
            node = node.right;
        }
        return true;
    }
}
