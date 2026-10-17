class Twitter {
    private int time = 0;
    // user -> {time, tweetId} pairs in posting order
    private final Map<Integer, List<int[]>> tweets = new HashMap<>();
    // user -> followees (never the user itself)
    private final Map<Integer, Set<Integer>> follows = new HashMap<>();

    public Twitter() {}

    public void postTweet(int userId, int tweetId) {
        tweets.computeIfAbsent(userId, k -> new ArrayList<>()).add(new int[] {time++, tweetId});
    }

    public List<Integer> getNewsFeed(int userId) {
        // Max-heap by time over {time, tweetId, user, index}: the newest unread tweet per user.
        PriorityQueue<int[]> heap = new PriorityQueue<>((x, y) -> y[0] - x[0]);
        Set<Integer> users = new HashSet<>(follows.getOrDefault(userId, Collections.emptySet()));
        users.add(userId);
        for (int u : users) {
            List<int[]> list = tweets.get(u);
            if (list == null || list.isEmpty()) continue;
            int i = list.size() - 1;
            heap.add(new int[] {list.get(i)[0], list.get(i)[1], u, i});
        }
        List<Integer> feed = new ArrayList<>();
        while (!heap.isEmpty() && feed.size() < 10) {
            int[] top = heap.poll();
            feed.add(top[1]);
            int i = top[3] - 1;
            if (i >= 0) {
                int[] next = tweets.get(top[2]).get(i);
                heap.add(new int[] {next[0], next[1], top[2], i});
            }
        }
        return feed;
    }

    public void follow(int followerId, int followeeId) {
        if (followerId != followeeId) {
            follows.computeIfAbsent(followerId, k -> new HashSet<>()).add(followeeId);
        }
    }

    public void unfollow(int followerId, int followeeId) {
        Set<Integer> set = follows.get(followerId);
        if (set != null) set.remove(followeeId);
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
