class Solution {
    public int sumEvenGrandparent(TreeNode root) {
        int total = 0;
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            for (TreeNode child : new TreeNode[] {node.left, node.right}) {
                if (child == null) continue;
                if (node.val % 2 == 0) {
                    if (child.left != null) total += child.left.val;
                    if (child.right != null) total += child.right.val;
                }
                stack.push(child);
            }
        }
        return total;
    }
}
