class Solution {
    public TreeNode balanceBst(TreeNode root) {
        List<Integer> values = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            values.add(node.val);
            node = node.right;
        }
        return build(values, 0, values.size() - 1);
    }

    private TreeNode build(List<Integer> values, int lo, int hi) {
        if (lo > hi) return null;
        int mid = (lo + hi) / 2;
        return new TreeNode(values.get(mid), build(values, lo, mid - 1), build(values, mid + 1, hi));
    }
}
