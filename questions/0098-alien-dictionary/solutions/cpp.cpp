class Solution {
public:
    string alienOrder(vector<string>& words) {
        set<char> letters;
        for (const string& w : words) letters.insert(w.begin(), w.end());
        map<char, set<char>> after;
        map<char, int> indegree;
        for (char ch : letters) {
            after[ch];
            indegree[ch] = 0;
        }
        for (size_t k = 0; k + 1 < words.size(); k++) {
            const string& first = words[k];
            const string& second = words[k + 1];
            size_t len = min(first.size(), second.size());
            bool differs = false;
            for (size_t i = 0; i < len; i++) {
                char a = first[i], b = second[i];
                if (a != b) {
                    if (after[a].insert(b).second) indegree[b]++;
                    differs = true;
                    break;
                }
            }
            if (!differs && first.size() > second.size()) return "";  // a longer word sits before its own prefix
        }
        priority_queue<char, vector<char>, greater<char>> heap;
        for (char ch : letters) {
            if (indegree[ch] == 0) heap.push(ch);
        }
        string order;
        while (!heap.empty()) {
            char ch = heap.top();
            heap.pop();
            order += ch;
            for (char nxt : after[ch]) {
                if (--indegree[nxt] == 0) heap.push(nxt);
            }
        }
        return order.size() == letters.size() ? order : "";
    }
};
