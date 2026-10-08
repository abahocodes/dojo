class Solution {
    public TreeNode increasingBst(TreeNode root) {
        TreeNode dummy = new TreeNode(0);
        TreeNode tail = dummy;
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            node.left = null;
            tail.right = node;
            tail = node;
            node = node.right;
        }
        return dummy.right;
    }
}
