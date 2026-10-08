class Solution {
    public int closestValue(TreeNode root, double target) {
        int best = root.val;
        TreeNode node = root;
        while (node != null) {
            int v = node.val;
            double d = Math.abs(v - target);
            double bd = Math.abs(best - target);
            if (d < bd || (d == bd && v < best)) best = v;
            if (target < v) node = node.left;
            else if (target > v) node = node.right;
            else break;
        }
        return best;
    }
}
