class Solution {
    public int averageOfSubtree(TreeNode root) {
        List<TreeNode> order = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            order.add(node);
            if (node.left != null) stack.push(node.left);
            if (node.right != null) stack.push(node.right);
        }
        Map<TreeNode, int[]> totals = new HashMap<>();
        int count = 0;
        for (int i = order.size() - 1; i >= 0; i--) {
            TreeNode node = order.get(i);
            int s = node.val, c = 1;
            for (TreeNode child : new TreeNode[] {node.left, node.right}) {
                if (child != null) {
                    int[] t = totals.get(child);
                    s += t[0];
                    c += t[1];
                }
            }
            totals.put(node, new int[] {s, c});
            if (s / c == node.val) count++;
        }
        return count;
    }
}
