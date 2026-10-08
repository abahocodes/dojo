class Solution {
public:
    int minimumRecolors(string& blocks, int k) {
        int whites = 0;
        for (int i = 0; i < k; i++) if (blocks[i] == 'W') whites++;
        int best = whites;
        for (int i = k; i < (int)blocks.size(); i++) {
            if (blocks[i] == 'W') whites++;
            if (blocks[i - k] == 'W') whites--;
            best = min(best, whites);
        }
        return best;
    }
};
