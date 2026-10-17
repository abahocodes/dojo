// 1006. LRU Cache (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class LRUCache {
public:
    LRUCache(int capacity) {

    }

    int get(int key) {
        return 0;
    }

    void put(int key, int value) {

    }
};

class Solution {
public:
    // dojo calls this to run the tests; you don't need to change it.
    vector<int> lruCache(int capacity, vector<string>& ops, vector<vector<int>>& args) {
        LRUCache cache(capacity);
        vector<int> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "put") cache.put(args[i][0], args[i][1]);
            else out.push_back(cache.get(args[i][0]));
        }
        return out;
    }
};
