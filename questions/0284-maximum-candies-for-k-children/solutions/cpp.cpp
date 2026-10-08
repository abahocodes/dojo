class Solution {
public:
    int maximumCandies(vector<int>& candies, long long k) {
        int lo = 0, hi = *max_element(candies.begin(), candies.end());
        while (lo < hi) {
            int mid = lo + (hi - lo + 1) / 2;
            long long shares = 0;
            for (int c : candies) {
                shares += c / mid;
                if (shares >= k) break;
            }
            if (shares >= k) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
};
