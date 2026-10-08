class Solution {
public:
    int kthSmallestMatrix(vector<vector<int>>& matrix, int k) {
        int n = matrix.size();
        // Values span up to 2 * 10^9, so do the midpoint arithmetic in long long.
        long long lo = matrix[0][0], hi = matrix[n - 1][n - 1];
        while (lo < hi) {
            long long mid = lo + (hi - lo) / 2;
            if (countAtMost(matrix, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return (int)lo;
    }

private:
    // Staircase walk from the bottom-left corner.
    int countAtMost(const vector<vector<int>>& matrix, long long v) {
        int n = matrix.size();
        int count = 0, row = n - 1, col = 0;
        while (row >= 0 && col < n) {
            if (matrix[row][col] <= v) {
                count += row + 1;
                col++;
            } else {
                row--;
            }
        }
        return count;
    }
};
