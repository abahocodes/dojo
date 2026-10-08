class Solution {
    public int diameterOfBinaryTree(TreeNode root) {
        if (root == null) return 0;
        // visit nodes so that children come before their parent (reversed pre-order)
        List<TreeNode> order = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            order.add(node);
            if (node.left != null) stack.push(node.left);
            if (node.right != null) stack.push(node.right);
        }
        Map<TreeNode, Integer> height = new HashMap<>(); // node -> number of nodes on its longest downward path
        int best = 0;
        for (int i = order.size() - 1; i >= 0; i--) {
            TreeNode node = order.get(i);
            int left = node.left != null ? height.get(node.left) : 0;
            int right = node.right != null ? height.get(node.right) : 0;
            best = Math.max(best, left + right);
            height.put(node, 1 + Math.max(left, right));
        }
        return best;
    }
}
