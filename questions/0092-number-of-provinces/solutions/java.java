class Solution {
    private int[] parent;

    private int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    public int findCircleNum(int[][] isConnected) {
        int n = isConnected.length;
        parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;
        int provinces = n;
        for (int i = 0; i < n; i++) {
            int[] row = isConnected[i];
            for (int j = i + 1; j < n; j++) {
                if (row[j] == 1) {
                    int ri = find(i), rj = find(j);
                    if (ri != rj) {
                        parent[ri] = rj;
                        provinces--;
                    }
                }
            }
        }
        return provinces;
    }
}
