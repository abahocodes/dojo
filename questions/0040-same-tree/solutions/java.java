class Solution {
    public boolean isSameTree(TreeNode p, TreeNode q) {
        List<TreeNode[]> stack = new ArrayList<>();
        stack.add(new TreeNode[] {p, q});
        while (!stack.isEmpty()) {
            TreeNode[] pair = stack.remove(stack.size() - 1);
            TreeNode a = pair[0], b = pair[1];
            if (a == null && b == null) continue;
            if (a == null || b == null || a.val != b.val) return false;
            stack.add(new TreeNode[] {a.left, b.left});
            stack.add(new TreeNode[] {a.right, b.right});
        }
        return true;
    }
}
