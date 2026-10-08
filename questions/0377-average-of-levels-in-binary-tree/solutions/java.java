class Solution {
    public double[] averageOfLevels(TreeNode root) {
        List<Double> averages = new ArrayList<>();
        Deque<TreeNode> queue = new ArrayDeque<>();
        queue.add(root);
        while (!queue.isEmpty()) {
            int size = queue.size();
            long total = 0;
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                total += node.val;
                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
            averages.add((double) total / size);
        }
        double[] result = new double[averages.size()];
        for (int i = 0; i < result.length; i++) result[i] = averages.get(i);
        return result;
    }
}
