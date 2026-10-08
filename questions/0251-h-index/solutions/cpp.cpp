class Solution {
public:
    int hIndex(vector<int>& citations) {
        int n = citations.size();
        vector<int> buckets(n + 1, 0);
        for (int c : citations) buckets[min(c, n)]++;
        int papers = 0;
        for (int h = n; h >= 0; h--) {
            papers += buckets[h];
            if (papers >= h) return h;
        }
        return 0;
    }
};
