class Solution {
    public long minMovesToEqual(int[] nums) {
        int[] a = nums.clone();
        int k = a.length / 2;
        int lo = 0, hi = a.length - 1;
        Random rng = new Random();
        while (lo < hi) {
            int pivot = a[lo + rng.nextInt(hi - lo + 1)];
            // Three-way partition: [lo, lt) < pivot, [lt, gt] == pivot, (gt, hi] > pivot
            int lt = lo, i = lo, gt = hi;
            while (i <= gt) {
                if (a[i] < pivot) {
                    int t = a[lt]; a[lt] = a[i]; a[i] = t;
                    lt++;
                    i++;
                } else if (a[i] > pivot) {
                    int t = a[gt]; a[gt] = a[i]; a[i] = t;
                    gt--;
                } else {
                    i++;
                }
            }
            if (k < lt) hi = lt - 1;
            else if (k > gt) lo = gt + 1;
            else break;
        }
        long median = a[k];
        long moves = 0;
        for (int v : a) moves += Math.abs(v - median);
        return moves;
    }
}
