class Solution {
    public boolean isCompleteTree(TreeNode root) {
        // ArrayDeque rejects nulls, so use a list with a read index as the queue.
        List<TreeNode> queue = new ArrayList<>();
        queue.add(root);
        boolean seenGap = false;
        for (int head = 0; head < queue.size(); head++) {
            TreeNode node = queue.get(head);
            if (node == null) {
                seenGap = true;
                continue;
            }
            if (seenGap) return false;
            queue.add(node.left);
            queue.add(node.right);
        }
        return true;
    }
}
