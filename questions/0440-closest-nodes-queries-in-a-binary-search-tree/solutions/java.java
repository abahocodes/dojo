class Solution {
    public int[][] closestNodes(TreeNode root, int[] queries) {
        List<Integer> list = new ArrayList<>();
        Deque<TreeNode> stack = new ArrayDeque<>();
        TreeNode node = root;
        while (!stack.isEmpty() || node != null) {
            while (node != null) {
                stack.push(node);
                node = node.left;
            }
            node = stack.pop();
            list.add(node.val);
            node = node.right;
        }
        int n = list.size();
        int[] values = new int[n];
        for (int i = 0; i < n; i++) values[i] = list.get(i);

        int[][] answer = new int[queries.length][];
        for (int qi = 0; qi < queries.length; qi++) {
            int q = queries[qi];
            int lo = 0, hi = n;
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (values[mid] < q) lo = mid + 1;
                else hi = mid;
            }
            if (lo < n && values[lo] == q) {
                answer[qi] = new int[] {q, q};
            } else {
                answer[qi] = new int[] {lo > 0 ? values[lo - 1] : -1, lo < n ? values[lo] : -1};
            }
        }
        return answer;
    }
}
