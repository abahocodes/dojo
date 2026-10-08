class Solution {
    public int containerWithMostWater(int[] heights) {
        int left = 0, right = heights.length - 1;
        int best = 0;
        while (left < right) {
            int width = right - left;
            if (heights[left] < heights[right]) {
                best = Math.max(best, width * heights[left]);
                left++;
            } else {
                best = Math.max(best, width * heights[right]);
                right--;
            }
        }
        return best;
    }
}
