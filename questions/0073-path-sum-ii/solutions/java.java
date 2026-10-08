class Solution {
    private record Frame(TreeNode node, boolean leaving) {}

    public int[][] pathSum(TreeNode root, int targetSum) {
        List<int[]> result = new ArrayList<>();
        if (root == null) return new int[0][];
        List<Integer> path = new ArrayList<>();
        long total = 0;
        // leaving=true means "undo this node on the way back up"
        Deque<Frame> stack = new ArrayDeque<>();
        stack.push(new Frame(root, false));
        while (!stack.isEmpty()) {
            Frame f = stack.pop();
            TreeNode node = f.node();
            if (f.leaving()) {
                path.remove(path.size() - 1);
                total -= node.val;
                continue;
            }
            path.add(node.val);
            total += node.val;
            stack.push(new Frame(node, true));
            if (node.left == null && node.right == null) {
                if (total == targetSum) result.add(path.stream().mapToInt(Integer::intValue).toArray());
            } else {
                // push right first so the left subtree is explored first
                if (node.right != null) stack.push(new Frame(node.right, false));
                if (node.left != null) stack.push(new Frame(node.left, false));
            }
        }
        return result.toArray(new int[0][]);
    }
}
