class Solution {
    public TreeNode createBinaryTree(int[][] descriptions) {
        Map<Integer, TreeNode> nodes = new HashMap<>();
        Set<Integer> children = new HashSet<>();
        for (int[] d : descriptions) {
            TreeNode parent = nodes.computeIfAbsent(d[0], TreeNode::new);
            TreeNode child = nodes.computeIfAbsent(d[1], TreeNode::new);
            if (d[2] == 1) parent.left = child;
            else parent.right = child;
            children.add(d[1]);
        }
        for (int[] d : descriptions) {
            if (!children.contains(d[0])) return nodes.get(d[0]);
        }
        return null;
    }
}
