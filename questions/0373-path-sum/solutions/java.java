class Solution {
    public boolean hasPathSum(TreeNode root, int targetSum) {
        if (root == null) return false;
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<Integer> remainders = new ArrayDeque<>();
        nodes.push(root);
        remainders.push(targetSum - root.val);
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            int remaining = remainders.pop();
            if (node.left == null && node.right == null) {
                if (remaining == 0) return true;
                continue;
            }
            if (node.left != null) {
                nodes.push(node.left);
                remainders.push(remaining - node.left.val);
            }
            if (node.right != null) {
                nodes.push(node.right);
                remainders.push(remaining - node.right.val);
            }
        }
        return false;
    }
}
