class Solution {
public:
    vector<int> sortColors(vector<int>& nums) {
        vector<int> a(nums);
        int low = 0, mid = 0, high = (int)a.size() - 1;
        while (mid <= high) {
            if (a[mid] == 0) {
                swap(a[low++], a[mid++]);
            } else if (a[mid] == 1) {
                mid++;
            } else {
                swap(a[mid], a[high--]);
            }
        }
        return a;
    }
};
