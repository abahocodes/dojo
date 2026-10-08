class Solution {
    public boolean isCousins(TreeNode root, int x, int y) {
        List<TreeNode> level = new ArrayList<>();
        level.add(root);
        while (!level.isEmpty()) {
            TreeNode parentX = null, parentY = null;
            List<TreeNode> next = new ArrayList<>();
            for (TreeNode node : level) {
                for (TreeNode child : new TreeNode[] {node.left, node.right}) {
                    if (child == null) continue;
                    if (child.val == x) parentX = node;
                    else if (child.val == y) parentY = node;
                    next.add(child);
                }
            }
            if (parentX != null && parentY != null) return parentX != parentY;
            if (parentX != null || parentY != null) return false;
            level = next;
        }
        return false;
    }
}
