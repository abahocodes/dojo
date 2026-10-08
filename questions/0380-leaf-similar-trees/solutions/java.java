class Solution {
    public boolean leafSimilar(TreeNode root1, TreeNode root2) {
        return leaves(root1).equals(leaves(root2));
    }

    private List<Integer> leaves(TreeNode root) {
        List<Integer> out = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        if (root != null) stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (node.left == null && node.right == null) {
                out.add(node.val);
                continue;
            }
            if (node.right != null) stack.push(node.right);
            if (node.left != null) stack.push(node.left);
        }
        return out;
    }
}
