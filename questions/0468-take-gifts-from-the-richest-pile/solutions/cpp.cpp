class Solution {
public:
    long long pickGifts(vector<int>& gifts, int k) {
        priority_queue<int> heap(gifts.begin(), gifts.end());
        for (int s = 0; s < k; s++) {
            int largest = heap.top();
            heap.pop();
            heap.push((int)sqrt((double)largest));
        }
        long long total = 0;
        while (!heap.empty()) {
            total += heap.top();
            heap.pop();
        }
        return total;
    }
};
