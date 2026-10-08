class Solution {
    public TreeNode convertBst(TreeNode root) {
        int running = 0;
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.right;
            }
            node = stack.pop();
            running += node.val;
            node.val = running;
            node = node.left;
        }
        return root;
    }
}
