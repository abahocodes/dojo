class Solution {
    public int[] rightSideView(TreeNode root) {
        if (root == null) return new int[0];
        List<Integer> view = new ArrayList<>();
        List<TreeNode> level = new ArrayList<>(List.of(root));
        while (!level.isEmpty()) {
            view.add(level.get(level.size() - 1).val);
            List<TreeNode> next = new ArrayList<>();
            for (TreeNode node : level) {
                if (node.left != null) next.add(node.left);
                if (node.right != null) next.add(node.right);
            }
            level = next;
        }
        return view.stream().mapToInt(Integer::intValue).toArray();
    }
}
