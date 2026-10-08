class Solution {
    private int[] parent;

    private int find(int x) {
        while (parent[x] != x) {
            parent[x] = parent[parent[x]];
            x = parent[x];
        }
        return x;
    }

    public String[][] accountsMerge(String[][] accounts) {
        int n = accounts.length;
        parent = new int[n];
        for (int i = 0; i < n; i++) parent[i] = i;

        Map<String, Integer> owner = new HashMap<>(); // email -> index of the first account that listed it
        for (int i = 0; i < n; i++) {
            for (int k = 1; k < accounts[i].length; k++) {
                String email = accounts[i][k];
                Integer j = owner.get(email);
                if (j != null) {
                    int ri = find(i), rj = find(j);
                    if (ri != rj) parent[ri] = rj;
                } else {
                    owner.put(email, i);
                }
            }
        }

        Map<Integer, List<String>> groups = new HashMap<>();
        for (Map.Entry<String, Integer> e : owner.entrySet()) {
            groups.computeIfAbsent(find(e.getValue()), r -> new ArrayList<>()).add(e.getKey());
        }

        List<String[]> merged = new ArrayList<>();
        for (Map.Entry<Integer, List<String>> e : groups.entrySet()) {
            List<String> emails = e.getValue();
            Collections.sort(emails);
            String[] account = new String[emails.size() + 1];
            account[0] = accounts[e.getKey()][0];
            for (int k = 0; k < emails.size(); k++) account[k + 1] = emails.get(k);
            merged.add(account);
        }
        merged.sort(Comparator.<String[], String>comparing(a -> a[0]).thenComparing(a -> a[1]));
        return merged.toArray(new String[0][]);
    }
}
