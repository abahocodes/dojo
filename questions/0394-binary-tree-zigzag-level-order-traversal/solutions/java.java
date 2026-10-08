class Solution {
    public int[][] zigzagLevelOrder(TreeNode root) {
        List<int[]> result = new ArrayList<>();
        Deque<TreeNode> queue = new ArrayDeque<>();
        if (root != null) queue.add(root);
        boolean leftToRight = true;
        while (!queue.isEmpty()) {
            int size = queue.size();
            int[] values = new int[size];
            for (int i = 0; i < size; i++) {
                TreeNode node = queue.poll();
                values[leftToRight ? i : size - 1 - i] = node.val;
                if (node.left != null) queue.add(node.left);
                if (node.right != null) queue.add(node.right);
            }
            result.add(values);
            leftToRight = !leftToRight;
        }
        return result.toArray(new int[0][]);
    }
}
