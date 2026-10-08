class Solution {
    public int[] getAllElements(TreeNode root1, TreeNode root2) {
        List<Integer> a = inorder(root1);
        List<Integer> b = inorder(root2);
        int[] merged = new int[a.size() + b.size()];
        int i = 0, j = 0, k = 0;
        while (i < a.size() && j < b.size()) {
            if (a.get(i) <= b.get(j)) merged[k++] = a.get(i++);
            else merged[k++] = b.get(j++);
        }
        while (i < a.size()) merged[k++] = a.get(i++);
        while (j < b.size()) merged[k++] = b.get(j++);
        return merged;
    }

    private List<Integer> inorder(TreeNode root) {
        List<Integer> values = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            values.add(node.val);
            node = node.right;
        }
        return values;
    }
}
