class Solution {
    public int subarraysDivByK(int[] nums, int k) {
        int[] count = new int[k];
        count[0] = 1;
        int rem = 0, result = 0;
        for (int x : nums) {
            rem = ((rem + x) % k + k) % k;
            result += count[rem];
            count[rem]++;
        }
        return result;
    }
}
