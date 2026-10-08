class Solution {
    public int[] closestKValues(TreeNode root, double target, int k) {
        // pred: path to values <= target (top = largest); succ: values > target (top = smallest).
        Deque<TreeNode> pred = new ArrayDeque<>();
        Deque<TreeNode> succ = new ArrayDeque<>();
        TreeNode node = root;
        while (node != null) {
            if (node.val <= target) {
                pred.push(node);
                node = node.right;
            } else {
                succ.push(node);
                node = node.left;
            }
        }
        int[] result = new int[k];
        for (int i = 0; i < k; i++) {
            if (succ.isEmpty() || (!pred.isEmpty() && target - pred.peek().val <= succ.peek().val - target)) {
                TreeNode top = pred.pop();
                for (TreeNode cur = top.left; cur != null; cur = cur.right) pred.push(cur);
                result[i] = top.val;
            } else {
                TreeNode top = succ.pop();
                for (TreeNode cur = top.right; cur != null; cur = cur.left) succ.push(cur);
                result[i] = top.val;
            }
        }
        return result;
    }
}
