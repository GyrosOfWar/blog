import { BlogPost } from '../components/BlogPost';
import * as qwest from 'qwest';

export class BlogPostStore {
    static readonly posts: Map<[number, number], BlogPost> = new Map();

    static getPost(userId: number, postId: number, successCb: ((post: BlogPost) => any), errorCb: ((message: string) => any)) {
        const entry = BlogPostStore.posts.get([userId, postId]);
        if (entry) {
            successCb(entry);
        } else {
            qwest.get(`/api/user/${userId}/post/${postId}`, null, { responseType: "json" })
                .then((xhr, resp) => {
                    if (resp.result) {
                        const post = BlogPost.fromJSON(resp.result);
                        successCb(post);
                        BlogPostStore.posts.set([userId, postId], post);
                    } else {
                        errorCb(resp || "Unknown error");
                    }
                })
                .catch((e, xhr, resp) => {
                    errorCb(resp.error.description as string);
                });
        }
    }
}