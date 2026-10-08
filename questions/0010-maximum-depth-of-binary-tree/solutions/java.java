class Solution {
    public int maxDepth(TreeNode root) {
        if (root == null) return 0;
        int depth = 0;
        List<TreeNode> level = new ArrayList<>();
        level.add(root);
        while (!level.isEmpty()) {
            depth++;
            List<TreeNode> nxt = new ArrayList<>();
            for (TreeNode node : level) {
                if (node.left != null) nxt.add(node.left);
                if (node.right != null) nxt.add(node.right);
            }
            level = nxt;
        }
        return depth;
    }
}
