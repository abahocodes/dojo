class Solution {
    private final List<List<Integer>> groups = new ArrayList<>();

    public int[][] findLeaves(TreeNode root) {
        groups.clear();
        height(root);
        int[][] result = new int[groups.size()][];
        for (int i = 0; i < result.length; i++) {
            List<Integer> group = groups.get(i);
            result[i] = new int[group.size()];
            for (int j = 0; j < group.size(); j++) result[i][j] = group.get(j);
        }
        return result;
    }

    private int height(TreeNode node) {
        if (node == null) return -1;
        int h = Math.max(height(node.left), height(node.right)) + 1;
        if (h == groups.size()) groups.add(new ArrayList<>());
        groups.get(h).add(node.val);
        return h;
    }
}
