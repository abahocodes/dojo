// 1009. Design Twitter (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class Twitter {
    constructor() {

    }

    postTweet(userId: number, tweetId: number): void {

    }

    getNewsFeed(userId: number): number[] {
        return [];
    }

    follow(followerId: number, followeeId: number): void {

    }

    unfollow(followerId: number, followeeId: number): void {

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
