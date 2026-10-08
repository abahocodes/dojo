class Solution {
    public int lowestCommonAncestor(TreeNode root, int p, int q) {
        int lo = Math.min(p, q), hi = Math.max(p, q);
        TreeNode node = root;
        while (true) {
            if (hi < node.val) {
                node = node.left;
            } else if (lo > node.val) {
                node = node.right;
            } else {
                return node.val;
            }
        }
    }
}
