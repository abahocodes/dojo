class Twitter {
    private time = 0;
    private tweets = new Map<number, [number, number][]>(); // user -> [time, tweetId] in posting order
    private follows = new Map<number, Set<number>>(); // user -> followees (never the user itself)

    postTweet(userId: number, tweetId: number): void {
        let list = this.tweets.get(userId);
        if (!list) {
            list = [];
            this.tweets.set(userId, list);
        }
        list.push([this.time++, tweetId]);
    }

    getNewsFeed(userId: number): number[] {
        // Only the last 10 tweets of each relevant user can make the feed.
        const users = new Set<number>(this.follows.get(userId) ?? []);
        users.add(userId);
        const pool: [number, number][] = [];
        for (const u of users) {
            const list = this.tweets.get(u) ?? [];
            for (let i = Math.max(0, list.length - 10); i < list.length; i++) pool.push(list[i]);
        }
        pool.sort((x, y) => y[0] - x[0]);
        return pool.slice(0, 10).map((t) => t[1]);
    }

    follow(followerId: number, followeeId: number): void {
        if (followerId === followeeId) return;
        let set = this.follows.get(followerId);
        if (!set) {
            set = new Set();
            this.follows.set(followerId, set);
        }
        set.add(followeeId);
    }

    unfollow(followerId: number, followeeId: number): void {
        this.follows.get(followerId)?.delete(followeeId);
    }
}

// dojo calls this to run the tests; you don't need to change it.
function runTwitter(ops: string[], args: number[][]): number[][] {
    const twitter = new Twitter();
    const out: number[][] = [];
    for (let i = 0; i < ops.length; i++) {
        const a = args[i];
        if (ops[i] === "postTweet") twitter.postTweet(a[0], a[1]);
        else if (ops[i] === "getNewsFeed") out.push(Array.from(twitter.getNewsFeed(a[0])));
        else if (ops[i] === "follow") twitter.follow(a[0], a[1]);
        else twitter.unfollow(a[0], a[1]);
    }
    return out;
}
