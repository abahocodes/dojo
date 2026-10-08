class Solution {
    private Map<Integer, List<TreeNode>> memo = new HashMap<>();

    public TreeNode[] generateTrees(int n) {
        memo.clear();
        return build(1, n).toArray(new TreeNode[0]);
    }

    private List<TreeNode> build(int lo, int hi) {
        List<TreeNode> trees = new ArrayList<>();
        if (lo > hi) {
            trees.add(null);
            return trees;
        }
        int key = lo * 100 + hi;
        List<TreeNode> cached = memo.get(key);
        if (cached != null) return cached;
        for (int v = lo; v <= hi; v++) {
            for (TreeNode left : build(lo, v - 1)) {
                for (TreeNode right : build(v + 1, hi)) {
                    trees.add(new TreeNode(v, left, right));
                }
            }
        }
        memo.put(key, trees);
        return trees;
    }
}
