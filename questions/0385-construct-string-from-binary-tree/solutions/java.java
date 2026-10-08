class Solution {
    public String tree2str(TreeNode root) {
        // the stack holds nodes still to print and literal text to emit
        StringBuilder out = new StringBuilder();
        Deque<Object> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            Object item = stack.pop();
            if (item instanceof String) {
                out.append((String) item);
                continue;
            }
            TreeNode node = (TreeNode) item;
            out.append(node.val);
            if (node.right != null) {
                stack.push(")");
                stack.push(node.right);
                stack.push("(");
            }
            if (node.left != null) {
                stack.push(")");
                stack.push(node.left);
                stack.push("(");
            } else if (node.right != null) {
                stack.push("()");
            }
        }
        return out.toString();
    }
}
