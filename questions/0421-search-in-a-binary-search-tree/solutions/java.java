class Solution {
    public TreeNode searchBst(TreeNode root, int val) {
        TreeNode node = root;
        while (node != null && node.val != val) {
            node = val < node.val ? node.left : node.right;
        }
        return node;
    }
}
