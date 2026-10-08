class Solution {
    public String getDirections(TreeNode root, int startValue, int destValue) {
        Map<Integer, Integer> parent = new HashMap<>();
        Map<Integer, Character> move = new HashMap<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty()) {
            TreeNode node = stack.pop();
            if (node.left != null) {
                parent.put(node.left.val, node.val);
                move.put(node.left.val, 'L');
                stack.push(node.left);
            }
            if (node.right != null) {
                parent.put(node.right.val, node.val);
                move.put(node.right.val, 'R');
                stack.push(node.right);
            }
        }
        String toStart = pathFromRoot(startValue, parent, move);
        String toDest = pathFromRoot(destValue, parent, move);
        int common = 0;
        while (common < toStart.length() && common < toDest.length()
                && toStart.charAt(common) == toDest.charAt(common)) {
            common++;
        }
        return "U".repeat(toStart.length() - common) + toDest.substring(common);
    }

    private String pathFromRoot(int value, Map<Integer, Integer> parent, Map<Integer, Character> move) {
        StringBuilder moves = new StringBuilder();
        while (parent.containsKey(value)) {
            moves.append(move.get(value));
            value = parent.get(value);
        }
        return moves.reverse().toString();
    }
}
