class LRUCache {
    int capacity;
    // Front is the most recently used, back the least.
    list<pair<int, int>> order;
    unordered_map<int, list<pair<int, int>>::iterator> nodes;

public:
    LRUCache(int capacity) : capacity(capacity) {}

    int get(int key) {
        auto it = nodes.find(key);
        if (it == nodes.end()) return -1;
        order.splice(order.begin(), order, it->second);  // O(1) move to front
        return it->second->second;
    }

    void put(int key, int value) {
        auto it = nodes.find(key);
        if (it != nodes.end()) {
            it->second->second = value;
            order.splice(order.begin(), order, it->second);
            return;
        }
        order.push_front({key, value});
        nodes[key] = order.begin();
        if ((int)nodes.size() > capacity) {
            nodes.erase(order.back().first);
            order.pop_back();
        }
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
