class Solution {
public:
    int longestMountain(vector<int>& arr) {
        int best = 0, up = 0, down = 0;
        for (size_t i = 1; i < arr.size(); i++) {
            if (arr[i - 1] == arr[i] || (down > 0 && arr[i - 1] < arr[i])) {
                up = 0;
                down = 0;
            }
            if (arr[i - 1] < arr[i]) up++;
            else if (arr[i - 1] > arr[i]) down++;
            if (up > 0 && down > 0) best = max(best, up + down + 1);
        }
        return best;
    }
};
