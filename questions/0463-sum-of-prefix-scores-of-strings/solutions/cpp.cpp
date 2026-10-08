class Solution {
public:
    vector<int> sumPrefixScores(vector<string>& words) {
        // Trie where every node counts how many words pass through it.
        vector<array<int, 26>> child(1);
        child[0].fill(0);
        vector<int> count(1, 0);
        for (const string& word : words) {
            int node = 0;
            for (char ch : word) {
                int c = ch - 'a';
                if (child[node][c] == 0) {
                    child[node][c] = (int)child.size();
                    child.emplace_back();
                    child.back().fill(0);
                    count.push_back(0);
                }
                node = child[node][c];
                count[node]++;
            }
        }

        vector<int> result;
        result.reserve(words.size());
        for (const string& word : words) {
            int node = 0, total = 0;
            for (char ch : word) {
                node = child[node][ch - 'a'];
                total += count[node];
            }
            result.push_back(total);
        }
        return result;
    }
};
