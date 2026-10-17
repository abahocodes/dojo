// 1003. Time Based Key-Value Store (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class TimeMap {
public:
    TimeMap() {

    }

    void set(string key, string value, int timestamp) {

    }

    string get(string key, int timestamp) {
        return "";
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
