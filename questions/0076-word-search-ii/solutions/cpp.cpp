class Solution {
    struct TrieNode {
        TrieNode* children[26] = {};
        int count = 0; // number of non-null children
        int word = -1; // index into words, or -1
    };

    vector<TrieNode> pool;
    vector<string> found;
    vector<string>* wordList = nullptr;
    vector<string> grid;
    int rows = 0, cols = 0;

    void dfs(int r, int c, TrieNode* parent) {
        char ch = grid[r][c];
        int k = ch - 'a';
        TrieNode* node = parent->children[k];
        if (node == nullptr) return;
        if (node->word >= 0) {
            found.push_back((*wordList)[node->word]);
            node->word = -1;
        }
        grid[r][c] = '#';
        static const int dr[] = {1, -1, 0, 0};
        static const int dc[] = {0, 0, 1, -1};
        for (int d = 0; d < 4; d++) {
            int nr = r + dr[d], nc = c + dc[d];
            if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr][nc] != '#') {
                dfs(nr, nc, node);
            }
        }
        grid[r][c] = ch;
        // prune branches with nothing left to find
        if (node->count == 0 && node->word < 0) {
            parent->children[k] = nullptr;
            parent->count--;
        }
    }

public:
    vector<string> findWords(vector<vector<string>>& board, vector<string>& words) {
        size_t total = 1;
        for (const string& w : words) total += w.size();
        pool.assign(total, TrieNode());
        size_t used = 1;
        TrieNode* root = &pool[0];
        for (int i = 0; i < (int)words.size(); i++) {
            TrieNode* node = root;
            for (char ch : words[i]) {
                int k = ch - 'a';
                if (node->children[k] == nullptr) {
                    node->children[k] = &pool[used++];
                    node->count++;
                }
                node = node->children[k];
            }
            node->word = i;
        }

        rows = board.size();
        cols = board[0].size();
        grid.assign(rows, string(cols, ' '));
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) grid[r][c] = board[r][c][0];
        }
        wordList = &words;
        found.clear();
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) dfs(r, c, root);
        }
        return found;
    }
};
