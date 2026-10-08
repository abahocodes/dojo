class Solution {
public:
    int oddEvenJumps(vector<int>& arr) {
        int n = arr.size();
        vector<int> byAsc(n), byDesc(n);
        iota(byAsc.begin(), byAsc.end(), 0);
        iota(byDesc.begin(), byDesc.end(), 0);
        sort(byAsc.begin(), byAsc.end(), [&](int a, int b) {
            return arr[a] != arr[b] ? arr[a] < arr[b] : a < b;
        });
        sort(byDesc.begin(), byDesc.end(), [&](int a, int b) {
            return arr[a] != arr[b] ? arr[a] > arr[b] : a < b;
        });
        vector<int> oddNext = targets(byAsc), evenNext = targets(byDesc);
        vector<bool> odd(n, false), even(n, false);
        odd[n - 1] = even[n - 1] = true;
        int count = 1;
        for (int i = n - 2; i >= 0; i--) {
            if (oddNext[i] != -1) odd[i] = even[oddNext[i]];
            if (evenNext[i] != -1) even[i] = odd[evenNext[i]];
            if (odd[i]) count++;
        }
        return count;
    }

private:
    // For each index, the first later entry of `order` that lies to its right.
    vector<int> targets(const vector<int>& order) {
        vector<int> nxt(order.size(), -1), stk;
        for (int j : order) {
            while (!stk.empty() && stk.back() < j) {
                nxt[stk.back()] = j;
                stk.pop_back();
            }
            stk.push_back(j);
        }
        return nxt;
    }
};
