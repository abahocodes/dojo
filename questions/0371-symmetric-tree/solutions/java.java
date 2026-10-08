class Solution {
    public boolean isSymmetric(TreeNode root) {
        Deque<TreeNode[]> stack = new ArrayDeque<>();
        stack.push(new TreeNode[] {root.left, root.right});
        while (!stack.isEmpty()) {
            TreeNode[] pair = stack.pop();
            TreeNode a = pair[0], b = pair[1];
            if (a == null && b == null) continue;
            if (a == null || b == null || a.val != b.val) return false;
            stack.push(new TreeNode[] {a.left, b.right});
            stack.push(new TreeNode[] {a.right, b.left});
        }
        return true;
    }
}
