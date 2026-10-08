class Solution {
    public int getMinimumDifference(TreeNode root) {
        int best = Integer.MAX_VALUE;
        boolean hasPrev = false;
        int prev = 0;
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            if (hasPrev) best = Math.min(best, node.val - prev);
            prev = node.val;
            hasPrev = true;
            node = node.right;
        }
        return best;
    }
}
