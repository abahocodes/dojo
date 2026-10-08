class Solution {
public:
    int trap(vector<int>& height) {
        int lo = 0, hi = (int)height.size() - 1;
        int leftMax = 0, rightMax = 0;
        int water = 0;
        while (lo <= hi) {
            if (leftMax <= rightMax) {
                leftMax = max(leftMax, height[lo]);
                water += leftMax - height[lo];
                lo++;
            } else {
                rightMax = max(rightMax, height[hi]);
                water += rightMax - height[hi];
                hi--;
            }
        }
        return water;
    }
};
