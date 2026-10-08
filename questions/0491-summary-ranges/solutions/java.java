class Solution {
    public String[] summaryRanges(int[] nums) {
        List<String> result = new ArrayList<>();
        int i = 0;
        while (i < nums.length) {
            int j = i;
            // nums[j] < nums[j + 1] <= 2^31 - 1, so nums[j] + 1 cannot overflow.
            while (j + 1 < nums.length && nums[j + 1] == nums[j] + 1) j++;
            result.add(i == j ? String.valueOf(nums[i]) : nums[i] + "->" + nums[j]);
            i = j + 1;
        }
        return result.toArray(new String[0]);
    }
}
