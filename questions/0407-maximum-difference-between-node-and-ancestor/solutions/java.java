class Solution {
    public int maxAncestorDiff(TreeNode root) {
        int best = 0;
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<int[]> bounds = new ArrayDeque<>();
        nodes.push(root);
        bounds.push(new int[] {root.val, root.val});
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            int[] b = bounds.pop();
            int lo = Math.min(b[0], node.val);
            int hi = Math.max(b[1], node.val);
            best = Math.max(best, hi - lo);
            if (node.left != null) {
                nodes.push(node.left);
                bounds.push(new int[] {lo, hi});
            }
            if (node.right != null) {
                nodes.push(node.right);
                bounds.push(new int[] {lo, hi});
            }
        }
        return best;
    }
}
