class Solution {
    public boolean isBalanced(TreeNode root) {
        if (root == null) return true;
        // reversed pre-order puts every child before its parent
        List<TreeNode> order = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            order.add(node);
            if (node.left != null) stack.push(node.left);
            if (node.right != null) stack.push(node.right);
        }
        Map<TreeNode, Integer> height = new HashMap<>();
        for (int i = order.size() - 1; i >= 0; i--) {
            TreeNode node = order.get(i);
            int left = node.left != null ? height.get(node.left) : 0;
            int right = node.right != null ? height.get(node.right) : 0;
            if (Math.abs(left - right) > 1) return false;
            height.put(node, 1 + Math.max(left, right));
        }
        return true;
    }
}
