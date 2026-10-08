class Solution {
    public TreeNode[] delNodes(TreeNode root, int[] toDelete) {
        Set<Integer> doomed = new HashSet<>();
        for (int v : toDelete) doomed.add(v);
        List<TreeNode> forest = new ArrayList<>();
        // Parallel stacks: the node, and whether it is the top of a tree once its parent is gone.
        Deque<TreeNode> stack = new ArrayDeque<>();
        Deque<Boolean> isRoot = new ArrayDeque<>();
        stack.push(root);
        isRoot.push(true);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            boolean top = isRoot.pop();
            boolean deleted = doomed.contains(node.val);
            if (top && !deleted) forest.add(node);
            if (node.left != null) {
                stack.push(node.left);
                isRoot.push(deleted);
                if (doomed.contains(node.left.val)) node.left = null;
            }
            if (node.right != null) {
                stack.push(node.right);
                isRoot.push(deleted);
                if (doomed.contains(node.right.val)) node.right = null;
            }
        }
        return forest.toArray(new TreeNode[0]);
    }
}
