class Solution {
    public int oddEvenJumps(int[] arr) {
        int n = arr.length;
        Integer[] byAsc = new Integer[n];
        Integer[] byDesc = new Integer[n];
        for (int i = 0; i < n; i++) {
            byAsc[i] = i;
            byDesc[i] = i;
        }
        Arrays.sort(byAsc, (a, b) -> arr[a] != arr[b] ? Integer.compare(arr[a], arr[b]) : Integer.compare(a, b));
        Arrays.sort(byDesc, (a, b) -> arr[a] != arr[b] ? Integer.compare(arr[b], arr[a]) : Integer.compare(a, b));
        int[] oddNext = targets(byAsc);
        int[] evenNext = targets(byDesc);
        boolean[] odd = new boolean[n];
        boolean[] even = new boolean[n];
        odd[n - 1] = even[n - 1] = true;
        int count = 1;
        for (int i = n - 2; i >= 0; i--) {
            if (oddNext[i] != -1) odd[i] = even[oddNext[i]];
            if (evenNext[i] != -1) even[i] = odd[evenNext[i]];
            if (odd[i]) count++;
        }
        return count;
    }

    // For each index, the first later entry of `order` that lies to its right.
    private int[] targets(Integer[] order) {
        int n = order.length;
        int[] nxt = new int[n];
        Arrays.fill(nxt, -1);
        int[] stack = new int[n];
        int top = 0;
        for (int j : order) {
            while (top > 0 && stack[top - 1] < j) nxt[stack[--top]] = j;
            stack[top++] = j;
        }
        return nxt;
    }
}
