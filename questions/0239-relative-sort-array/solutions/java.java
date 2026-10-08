class Solution {
    public int[] relativeSortArray(int[] arr1, int[] arr2) {
        int[] count = new int[1001];
        for (int x : arr1) count[x]++;
        int[] result = new int[arr1.length];
        int k = 0;
        for (int x : arr2) {
            while (count[x] > 0) {
                result[k++] = x;
                count[x]--;
            }
        }
        for (int x = 0; x <= 1000; x++) {
            while (count[x] > 0) {
                result[k++] = x;
                count[x]--;
            }
        }
        return result;
    }
}
