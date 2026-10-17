// 1009. Design Twitter (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Twitter {
    public Twitter() {

    }

    public void postTweet(int userId, int tweetId) {

    }

    public List<Integer> getNewsFeed(int userId) {
        return new ArrayList<>();
    }

    public void follow(int followerId, int followeeId) {

    }

    public void unfollow(int followerId, int followeeId) {

    }
}

// dojo calls this to run the tests; you don't need to change it.
class Solution {
    public int[][] runTwitter(String[] ops, int[][] args) {
        Twitter twitter = new Twitter();
        List<int[]> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            int[] a = args[i];
            switch (ops[i]) {
                case "postTweet": twitter.postTweet(a[0], a[1]); break;
                case "getNewsFeed": {
                    List<Integer> feed = twitter.getNewsFeed(a[0]);
                    int[] row = new int[feed == null ? 0 : feed.size()];
                    for (int j = 0; j < row.length; j++) row[j] = feed.get(j);
                    out.add(row);
                    break;
                }
                case "follow": twitter.follow(a[0], a[1]); break;
                default: twitter.unfollow(a[0], a[1]);
            }
        }
        return out.toArray(new int[0][]);
    }
}
