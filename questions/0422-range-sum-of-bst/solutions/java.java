class Solution {
    public int rangeSumBst(TreeNode root, int low, int high) {
        int total = 0;
        Deque<TreeNode> stack = new ArrayDeque<>();
        if (root != null) stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (node.val < low) {
                if (node.right != null) stack.push(node.right);
            } else if (node.val > high) {
                if (node.left != null) stack.push(node.left);
            } else {
                total += node.val;
                if (node.left != null) stack.push(node.left);
                if (node.right != null) stack.push(node.right);
            }
        }
        return total;
    }
}
