class Solution {
    public int findTilt(TreeNode root) {
        if (root == null) return 0;
        List<TreeNode> order = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            order.add(node);
            if (node.left != null) stack.push(node.left);
            if (node.right != null) stack.push(node.right);
        }
        Map<TreeNode, Integer> subtreeSum = new HashMap<>();
        int tilt = 0;
        for (int i = order.size() - 1; i >= 0; i--) {
            TreeNode node = order.get(i);
            int left = node.left != null ? subtreeSum.get(node.left) : 0;
            int right = node.right != null ? subtreeSum.get(node.right) : 0;
            tilt += Math.abs(left - right);
            subtreeSum.put(node, node.val + left + right);
        }
        return tilt;
    }
}
