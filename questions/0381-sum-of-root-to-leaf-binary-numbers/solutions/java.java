class Solution {
    public int sumRootToLeaf(TreeNode root) {
        int total = 0;
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<Integer> prefixes = new ArrayDeque<>();
        if (root != null) {
            nodes.push(root);
            prefixes.push(0);
        }
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            int value = prefixes.pop() * 2 + node.val;
            if (node.left == null && node.right == null) {
                total += value;
                continue;
            }
            if (node.left != null) { nodes.push(node.left); prefixes.push(value); }
            if (node.right != null) { nodes.push(node.right); prefixes.push(value); }
        }
        return total;
    }
}
