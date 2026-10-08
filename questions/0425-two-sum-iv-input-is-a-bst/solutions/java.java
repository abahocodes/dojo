class Solution {
    public boolean findTarget(TreeNode root, int k) {
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

        int i = 0, j = values.size() - 1;
        while (i < j) {
            int s = values.get(i) + values.get(j);
            if (s == k) return true;
            if (s < k) i++;
            else j--;
        }
        return false;
    }
}
