class Solution {
    public boolean evaluateTree(TreeNode root) {
        Map<TreeNode, Boolean> value = new HashMap<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        Deque<Boolean> done = new ArrayDeque<>();
        stack.push(root);
        done.push(false);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            boolean childrenDone = done.pop();
            if (node.left == null) {
                value.put(node, node.val == 1);
            } else if (childrenDone) {
                boolean left = value.get(node.left);
                boolean right = value.get(node.right);
                value.put(node, node.val == 2 ? (left || right) : (left && right));
            } else {
                stack.push(node);
                done.push(true);
                stack.push(node.left);
                done.push(false);
                stack.push(node.right);
                done.push(false);
            }
        }
        return value.get(root);
    }
}
