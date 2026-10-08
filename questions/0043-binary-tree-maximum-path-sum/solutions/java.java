class Solution {
    public int maxPathSum(TreeNode root) {
        List<TreeNode> order = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            order.add(node);
            if (node.left != null) stack.push(node.left);
            if (node.right != null) stack.push(node.right);
        }

        Map<TreeNode, Integer> gain = new HashMap<>();
        int best = root.val;
        for (int k = order.size() - 1; k >= 0; k--) {
            TreeNode node = order.get(k);
            int left = Math.max(gain.getOrDefault(node.left, 0), 0);
            int right = Math.max(gain.getOrDefault(node.right, 0), 0);
            best = Math.max(best, node.val + left + right);
            gain.put(node, node.val + Math.max(left, right));
        }
        return best;
    }
}
