class Solution {
    public int goodNodes(TreeNode root) {
        if (root == null) return 0;
        int count = 0;
        // parallel stacks: a node and the largest value strictly above it on the path
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<Integer> bests = new ArrayDeque<>();
        nodes.push(root);
        bests.push(root.val);
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            int best = bests.pop();
            if (node.val >= best) {
                count++;
                best = node.val;
            }
            if (node.left != null) {
                nodes.push(node.left);
                bests.push(best);
            }
            if (node.right != null) {
                nodes.push(node.right);
                bests.push(best);
            }
        }
        return count;
    }
}
