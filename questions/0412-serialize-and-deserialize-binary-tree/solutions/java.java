class Solution {
    public TreeNode deserialize(String data) {
        String[] tokens = data.split(",");
        if (tokens[0].equals("#")) return null;
        TreeNode root = new TreeNode(Integer.parseInt(tokens[0]));
        Deque<TreeNode> stack = new ArrayDeque<>();
        Deque<Boolean> leftDone = new ArrayDeque<>();
        stack.push(root);
        leftDone.push(false);
        for (int i = 1; i < tokens.length; i++) {
            TreeNode child = tokens[i].equals("#") ? null : new TreeNode(Integer.parseInt(tokens[i]));
            TreeNode parent = stack.peek();
            if (!leftDone.peek()) {
                parent.left = child;
                leftDone.pop();
                leftDone.push(true);
            } else {
                parent.right = child;
                stack.pop();
                leftDone.pop();
            }
            if (child != null) {
                stack.push(child);
                leftDone.push(false);
            }
        }
        return root;
    }
}
