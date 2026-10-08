class Solution {
    public int[][] verticalTraversal(TreeNode root) {
        List<int[]> entries = new ArrayList<>(); // {col, row, val}
        Deque<TreeNode> nodes = new ArrayDeque<>();
        Deque<int[]> pos = new ArrayDeque<>();
        nodes.push(root);
        pos.push(new int[]{0, 0});
        while (!nodes.isEmpty()) {
            TreeNode node = nodes.pop();
            int[] p = pos.pop();
            int row = p[0], col = p[1];
            entries.add(new int[]{col, row, node.val});
            if (node.left != null) {
                nodes.push(node.left);
                pos.push(new int[]{row + 1, col - 1});
            }
            if (node.right != null) {
                nodes.push(node.right);
                pos.push(new int[]{row + 1, col + 1});
            }
        }
        entries.sort((a, b) -> a[0] != b[0] ? Integer.compare(a[0], b[0])
                : a[1] != b[1] ? Integer.compare(a[1], b[1])
                : Integer.compare(a[2], b[2]));
        List<int[]> result = new ArrayList<>();
        int i = 0;
        while (i < entries.size()) {
            int j = i;
            while (j < entries.size() && entries.get(j)[0] == entries.get(i)[0]) j++;
            int[] column = new int[j - i];
            for (int k = i; k < j; k++) column[k - i] = entries.get(k)[2];
            result.add(column);
            i = j;
        }
        return result.toArray(new int[0][]);
    }
}
