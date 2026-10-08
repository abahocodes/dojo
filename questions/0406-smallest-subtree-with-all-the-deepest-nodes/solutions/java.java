class Solution {
    private static final class Result {
        final int height;
        final TreeNode node;

        Result(int height, TreeNode node) {
            this.height = height;
            this.node = node;
        }
    }

    public TreeNode subtreeWithAllDeepest(TreeNode root) {
        return dfs(root).node;
    }

    private Result dfs(TreeNode node) {
        if (node == null) return new Result(0, null);
        Result left = dfs(node.left);
        Result right = dfs(node.right);
        if (left.height > right.height) return new Result(left.height + 1, left.node);
        if (right.height > left.height) return new Result(right.height + 1, right.node);
        return new Result(left.height + 1, node);
    }
}
