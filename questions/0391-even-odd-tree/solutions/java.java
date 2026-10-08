class Solution {
    public boolean isEvenOddTree(TreeNode root) {
        Deque<TreeNode> queue = new ArrayDeque<>();
        queue.add(root);
        int depth = 0;
        while (!queue.isEmpty()) {
            boolean evenLevel = depth % 2 == 0;
            // Sentinel just outside the value range [1, 10^6].
            int prev = evenLevel ? 0 : Integer.MAX_VALUE;
            int size = queue.size();
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                int v = node.val;
                if (evenLevel) {
                    if (v % 2 == 0 || v <= prev) return false;
                } else {
                    if (v % 2 == 1 || v >= prev) return false;
                }
                prev = v;
                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
            depth++;
        }
        return true;
    }
}
