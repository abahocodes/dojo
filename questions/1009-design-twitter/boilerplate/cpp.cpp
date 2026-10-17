// 1009. Design Twitter (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Twitter {
public:
    Twitter() {

    }

    void postTweet(int userId, int tweetId) {

    }

    vector<int> getNewsFeed(int userId) {
        return {};
    }

    void follow(int followerId, int followeeId) {

    }

    void unfollow(int followerId, int followeeId) {

    }
};

// dojo calls this to run the tests; you don't need to change it.
class Solution {
public:
    vector<vector<int>> runTwitter(vector<string>& ops, vector<vector<int>>& args) {
        Twitter twitter;
        vector<vector<int>> out;
        for (size_t i = 0; i < ops.size(); i++) {
            const vector<int>& a = args[i];
            if (ops[i] == "postTweet") twitter.postTweet(a[0], a[1]);
            else if (ops[i] == "getNewsFeed") out.push_back(twitter.getNewsFeed(a[0]));
            else if (ops[i] == "follow") twitter.follow(a[0], a[1]);
            else twitter.unfollow(a[0], a[1]);
        }
        return out;
    }
};
