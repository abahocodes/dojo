class Solution {
    public int[][] minimumAbsDifference(int[] arr) {
        int[] a = arr.clone();
        Arrays.sort(a);
        int best = Integer.MAX_VALUE;
        for (int i = 0; i + 1 < a.length; i++) best = Math.min(best, a[i + 1] - a[i]);
        List<int[]> out = new ArrayList<>();
        for (int i = 0; i + 1 < a.length; i++) {
            if (a[i + 1] - a[i] == best) out.add(new int[] {a[i], a[i + 1]});
        }
        return out.toArray(new int[0][]);
    }
}
