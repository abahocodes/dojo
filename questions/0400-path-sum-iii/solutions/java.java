class Solution {
    public int pathSumCount(TreeNode root, int targetSum) {
        if (root == null) return 0;
        Map<Long, Integer> seen = new HashMap<>(); // prefix sums on the current root-to-node path
        seen.put(0L, 1);
        int count = 0;
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<Long> before = new ArrayDeque<>();
        Deque<Boolean> leaving = new ArrayDeque<>();
        nodes.push(root);
        before.push(0L);
        leaving.push(false);
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            long prefix = before.pop() + node.val;
            if (leaving.pop()) {
                seen.merge(prefix, -1, Integer::sum);
                continue;
            }
            count += seen.getOrDefault(prefix - targetSum, 0);
            seen.merge(prefix, 1, Integer::sum);
            nodes.push(node);
            before.push(prefix - node.val);
            leaving.push(true);
            if (node.right != null) {
                nodes.push(node.right);
                before.push(prefix);
                leaving.push(false);
            }
            if (node.left != null) {
                nodes.push(node.left);
                before.push(prefix);
                leaving.push(false);
            }
        }
        return count;
    }
}
