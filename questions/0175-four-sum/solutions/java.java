class Solution {
    public int[][] fourSum(int[] nums, int target) {
        int[] a = nums.clone();
        Arrays.sort(a);
        int n = a.length;
        List<int[]> res = new ArrayList<>();
        for (int i = 0; i < n - 3; i++) {
            if (i > 0 && a[i] == a[i - 1]) continue;
            for (int j = i + 1; j < n - 2; j++) {
                if (j > i + 1 && a[j] == a[j - 1]) continue;
                int lo = j + 1, hi = n - 1;
                while (lo < hi) {
                    long s = (long) a[i] + a[j] + a[lo] + a[hi];
                    if (s < target) lo++;
                    else if (s > target) hi--;
                    else {
                        res.add(new int[] {a[i], a[j], a[lo], a[hi]});
                        lo++;
                        while (lo < hi && a[lo] == a[lo - 1]) lo++;
                        hi--;
                        while (lo < hi && a[hi] == a[hi + 1]) hi--;
                    }
                }
            }
        }
        return res.toArray(new int[0][]);
    }
}
