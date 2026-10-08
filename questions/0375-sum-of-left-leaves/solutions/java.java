class Solution {
    public int sumOfLeftLeaves(TreeNode root) {
        int total = 0;
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            TreeNode left = node.left;
            if (left != null) {
                if (left.left == null && left.right == null) total += left.val;
                else stack.push(left);
            }
            if (node.right != null) stack.push(node.right);
        }
        return total;
    }
}
