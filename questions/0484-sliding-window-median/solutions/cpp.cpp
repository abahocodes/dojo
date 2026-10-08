class Solution {
    priority_queue<int> low;                            // max-heap: smaller half
    priority_queue<int, vector<int>, greater<int>> high; // min-heap: larger half
    unordered_map<int, int> delayed; // value -> copies removed but still inside a heap
    int lowSize = 0;
    int highSize = 0;

    template <typename Heap>
    void prune(Heap& heap) {
        while (!heap.empty()) {
            auto it = delayed.find(heap.top());
            if (it == delayed.end() || it->second == 0) break;
            it->second--;
            heap.pop();
        }
    }

    void rebalance() {
        if (lowSize > highSize + 1) {
            high.push(low.top());
            low.pop();
            lowSize--;
            highSize++;
            prune(low);
        } else if (lowSize < highSize) {
            low.push(high.top());
            high.pop();
            lowSize++;
            highSize--;
            prune(high);
        }
    }

    void add(int x) {
        if (low.empty() || x <= low.top()) {
            low.push(x);
            lowSize++;
        } else {
            high.push(x);
            highSize++;
        }
        rebalance();
    }

    void remove(int x) {
        delayed[x]++;
        if (x <= low.top()) {
            lowSize--;
            if (x == low.top()) prune(low);
        } else {
            highSize--;
            if (!high.empty() && x == high.top()) prune(high);
        }
        rebalance();
    }

public:
    vector<double> medianSlidingWindow(vector<int>& nums, int k) {
        low = priority_queue<int>();
        high = priority_queue<int, vector<int>, greater<int>>();
        delayed.clear();
        lowSize = highSize = 0;

        vector<double> result;
        for (int i = 0; i < (int)nums.size(); i++) {
            add(nums[i]);
            if (i >= k) remove(nums[i - k]);
            if (i >= k - 1) {
                if (k % 2 == 1) {
                    result.push_back((double)low.top());
                } else {
                    result.push_back(((double)low.top() + (double)high.top()) / 2.0);
                }
            }
        }
        return result;
    }
};
