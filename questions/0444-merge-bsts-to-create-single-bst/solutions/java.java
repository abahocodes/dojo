class Solution {
    public TreeNode canMerge(TreeNode[] trees) {
        Map<Integer, TreeNode> roots = new HashMap<>();
        Set<Integer> leafValues = new HashSet<>();
        for (TreeNode t : trees) {
            roots.put(t.val, t);
            if (t.left != null) leafValues.add(t.left.val);
            if (t.right != null) leafValues.add(t.right.val);
        }

        TreeNode root = null;
        int candidates = 0;
        for (TreeNode t : trees) {
            if (!leafValues.contains(t.val)) {
                root = t;
                candidates++;
            }
        }
        if (candidates != 1) return null;
        roots.remove(root.val);

        // Iterative DFS carrying the open interval (lo, hi) each node must fit in.
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<long[]> bounds = new ArrayDeque<>();
        nodes.push(root);
        bounds.push(new long[] {0, 1L << 31});
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            long[] b = bounds.pop();
            if (node.val <= b[0] || node.val >= b[1]) return null;
            if (node.left == null && node.right == null && roots.containsKey(node.val)) {
                TreeNode sub = roots.remove(node.val);
                node.left = sub.left;
                node.right = sub.right;
            }
            if (node.left != null) {
                nodes.push(node.left);
                bounds.push(new long[] {b[0], node.val});
            }
            if (node.right != null) {
                nodes.push(node.right);
                bounds.push(new long[] {node.val, b[1]});
            }
        }

        return roots.isEmpty() ? root : null;
    }
}
