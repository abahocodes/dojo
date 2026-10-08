class Solution {
    public int findSecondMinimumValue(TreeNode root) {
        int smallest = root.val;
        int best = -1;
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (node.val > smallest) {
                if (best == -1 || node.val < best) best = node.val;
                continue;
            }
            if (node.left != null) {
                stack.push(node.left);
                stack.push(node.right);
            }
        }
        return best;
    }
}
