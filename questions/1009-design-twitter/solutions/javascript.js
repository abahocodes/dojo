class Twitter {
  constructor() {
    this.time = 0;
    this.tweets = new Map(); // user -> [[time, tweetId]] in posting order
    this.follows = new Map(); // user -> Set of followees (never the user itself)
  }

  postTweet(userId, tweetId) {
    if (!this.tweets.has(userId)) this.tweets.set(userId, []);
    this.tweets.get(userId).push([this.time++, tweetId]);
  }

  getNewsFeed(userId) {
    // Only the last 10 tweets of each relevant user can make the feed.
    const users = new Set(this.follows.get(userId) || []);
    users.add(userId);
    const pool = [];
    for (const u of users) {
      const list = this.tweets.get(u) || [];
      for (let i = Math.max(0, list.length - 10); i < list.length; i++) pool.push(list[i]);
    }
    pool.sort((x, y) => y[0] - x[0]);
    return pool.slice(0, 10).map((t) => t[1]);
  }

  follow(followerId, followeeId) {
    if (followerId === followeeId) return;
    if (!this.follows.has(followerId)) this.follows.set(followerId, new Set());
    this.follows.get(followerId).add(followeeId);
  }

  unfollow(followerId, followeeId) {
    const set = this.follows.get(followerId);
    if (set) set.delete(followeeId);
  }
}

// dojo calls this to run the tests; you don't need to change it.
function runTwitter(ops, args) {
  const twitter = new Twitter();
  const out = [];
  for (let i = 0; i < ops.length; i++) {
    const a = args[i];
    if (ops[i] === "postTweet") twitter.postTweet(a[0], a[1]);
    else if (ops[i] === "getNewsFeed") out.push(Array.from(twitter.getNewsFeed(a[0])));
    else if (ops[i] === "follow") twitter.follow(a[0], a[1]);
    else twitter.unfollow(a[0], a[1]);
  }
  return out;
}
