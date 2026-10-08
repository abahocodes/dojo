class Solution {
    public int lowestCommonAncestor(TreeNode root, int p, int q) {
        Map<Integer, TreeNode> parent = new HashMap<>();
        parent.put(root.val, null);
        Deque<TreeNode> stack = new ArrayDeque<>();
        stack.push(root);
        while (!stack.isEmpty() && !(parent.containsKey(p) && parent.containsKey(q))) {
            TreeNode node = stack.pop();
            if (node.left != null) {
                parent.put(node.left.val, node);
                stack.push(node.left);
            }
            if (node.right != null) {
                parent.put(node.right.val, node);
                stack.push(node.right);
            }
        }
        Set<Integer> ancestors = new HashSet<>();
        ancestors.add(p);
        for (TreeNode up = parent.get(p); up != null; up = parent.get(up.val)) {
            ancestors.add(up.val);
        }
        int v = q;
        while (!ancestors.contains(v)) v = parent.get(v).val;
        return v;
    }
}
