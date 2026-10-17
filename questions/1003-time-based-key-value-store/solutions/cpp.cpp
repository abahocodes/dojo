class TimeMap {
    // key -> (timestamp, value) writes; set timestamps only increase, so each list is sorted.
    unordered_map<string, vector<pair<int, string>>> history;

public:
    TimeMap() {}

    void set(string key, string value, int timestamp) {
        history[key].push_back({timestamp, value});
    }

    string get(string key, int timestamp) {
        auto it = history.find(key);
        if (it == history.end()) return "";
        const auto& h = it->second;
        // First write after `timestamp`; the one before it is the answer.
        auto after = upper_bound(h.begin(), h.end(), timestamp,
                                 [](int t, const pair<int, string>& e) { return t < e.first; });
        return after == h.begin() ? "" : prev(after)->second;
    }
};

class Solution {
public:
    // dojo calls this to run the tests; you don't need to change it.
    vector<string> timeMap(vector<string>& ops, vector<string>& keys, vector<string>& values,
                           vector<int>& timestamps) {
        TimeMap store;
        vector<string> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "set") store.set(keys[i], values[i], timestamps[i]);
            else out.push_back(store.get(keys[i], timestamps[i]));
        }
        return out;
    }
};
