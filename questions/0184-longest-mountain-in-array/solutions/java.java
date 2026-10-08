class Solution {
    public int longestMountain(int[] arr) {
        int best = 0, up = 0, down = 0;
        for (int i = 1; i < arr.length; i++) {
            if (arr[i - 1] == arr[i] || (down > 0 && arr[i - 1] < arr[i])) {
                up = 0;
                down = 0;
            }
            if (arr[i - 1] < arr[i]) up++;
            else if (arr[i - 1] > arr[i]) down++;
            if (up > 0 && down > 0) best = Math.max(best, up + down + 1);
        }
        return best;
    }
}
