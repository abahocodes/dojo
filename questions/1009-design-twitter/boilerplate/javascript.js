// 1009. Design Twitter (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Twitter {
  constructor() {

  }

  /** @param {number} userId @param {number} tweetId */
  postTweet(userId, tweetId) {

  }

  /** @param {number} userId @return {number[]} */
  getNewsFeed(userId) {
    return [];
  }

  /** @param {number} followerId @param {number} followeeId */
  follow(followerId, followeeId) {

  }

  /** @param {number} followerId @param {number} followeeId */
  unfollow(followerId, followeeId) {

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
