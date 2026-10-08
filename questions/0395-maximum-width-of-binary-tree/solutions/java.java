class Solution {
    public int widthOfBinaryTree(TreeNode root) {
        long best = 0;
        List<TreeNode> nodes = new ArrayList<>();
        List<Long> positions = new ArrayList<>();
        nodes.add(root);
        positions.add(0L);
        while (!nodes.isEmpty()) {
            long base = positions.get(0);
            best = Math.max(best, positions.get(positions.size() - 1) - base + 1);
            List<TreeNode> nextNodes = new ArrayList<>();
            List<Long> nextPositions = new ArrayList<>();
            for (int i = 0; i < nodes.size(); i++) {
                TreeNode node = nodes.get(i);
                long pos = positions.get(i) - base;
                if (node.left != null) {
                    nextNodes.add(node.left);
                    nextPositions.add(2 * pos);
                }
                if (node.right != null) {
                    nextNodes.add(node.right);
                    nextPositions.add(2 * pos + 1);
                }
            }
            nodes = nextNodes;
            positions = nextPositions;
        }
        return (int) best;
    }
}
