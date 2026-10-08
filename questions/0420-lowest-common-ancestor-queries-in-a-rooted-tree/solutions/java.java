class Solution {
    public int[] lcaQueries(int[] parent, int[][] queries) {
        int n = parent.length;
        int root = 0;
        // Children in CSR form: the children of u are kids[start[u] .. start[u + 1]).
        int[] start = new int[n + 1];
        for (int v = 0; v < n; v++) {
            if (parent[v] == -1) root = v;
            else start[parent[v] + 1]++;
        }
        for (int i = 0; i < n; i++) start[i + 1] += start[i];
        int[] fill = Arrays.copyOf(start, n);
        int[] kids = new int[n];
        for (int v = 0; v < n; v++) {
            if (parent[v] != -1) kids[fill[parent[v]]++] = v;
        }

        int[] depth = new int[n];
        int[] queue = new int[n];
        int head = 0, tail = 0;
        queue[tail++] = root;
        while (head < tail) {
            int u = queue[head++];
            for (int i = start[u]; i < start[u + 1]; i++) {
                int c = kids[i];
                depth[c] = depth[u] + 1;
                queue[tail++] = c;
            }
        }

        int log = 1;
        while ((1 << log) < n) log++;
        int[][] up = new int[log][n];
        for (int v = 0; v < n; v++) up[0][v] = parent[v] == -1 ? root : parent[v];
        for (int k = 1; k < log; k++) {
            for (int v = 0; v < n; v++) up[k][v] = up[k - 1][up[k - 1][v]];
        }

        int[] answers = new int[queries.length];
        for (int i = 0; i < queries.length; i++) {
            int u = queries[i][0], v = queries[i][1];
            if (depth[u] < depth[v]) {
                int t = u;
                u = v;
                v = t;
            }
            int diff = depth[u] - depth[v];
            for (int k = 0; diff > 0; k++, diff >>= 1) {
                if ((diff & 1) != 0) u = up[k][u];
            }
            if (u != v) {
                for (int k = log - 1; k >= 0; k--) {
                    if (up[k][u] != up[k][v]) {
                        u = up[k][u];
                        v = up[k][v];
                    }
                }
                u = up[0][u];
            }
            answers[i] = u;
        }
        return answers;
    }
}
