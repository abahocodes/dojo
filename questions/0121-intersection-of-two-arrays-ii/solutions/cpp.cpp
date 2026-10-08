class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        vector<int> counts(1001, 0);
        for (int x : nums1) counts[x]++;
        vector<int> out;
        for (int x : nums2) {
            if (counts[x] > 0) {
                counts[x]--;
                out.push_back(x);
            }
        }
        return out;
    }
};
