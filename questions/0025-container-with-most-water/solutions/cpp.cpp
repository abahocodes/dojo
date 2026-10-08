class Solution {
public:
    int containerWithMostWater(vector<int>& heights) {
        int left = 0, right = (int)heights.size() - 1;
        int best = 0;
        while (left < right) {
            int width = right - left;
            if (heights[left] < heights[right]) {
                best = max(best, width * heights[left]);
                left++;
            } else {
                best = max(best, width * heights[right]);
                right--;
            }
        }
        return best;
    }
};
