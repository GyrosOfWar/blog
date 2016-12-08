import * as React from "react";
import { Link } from "react-router";
import { BlogPostStore } from "../store/BlogPostStore";

enum LoadState {
    Loading = 0,
    Finished = 1,
    Error = 2
}

export class BlogPost {
    readonly title: string;
    readonly content: string;
    readonly id: number;
    readonly createdOn: Date;
    readonly ownerId: number;
    readonly tags: Array<string>;

    constructor(title: string, content: string, id: number, createdOn: Date, ownerId: number, tags: Array<string>) {
        this.title = title;
        this.content = content;
        this.id = id;
        this.createdOn = createdOn;
        this.ownerId = ownerId;
        this.tags = tags;
    }

    static fromJSON(obj: any): BlogPost {
        return new BlogPost(
            obj.title,
            obj.content,
            obj.id,
            new Date(obj.created_on),
            obj.owner_id,
            obj.tags
        );
    }
}

interface State {
    post?: BlogPost,
    error?: string,
    loadState: LoadState,
}

class Loading extends React.Component<{}, {}> {
    render(): JSX.Element {
        return (
            <div className="spinner">
                <div className="double-bounce1"></div>
                <div className="double-bounce2"></div>
            </div>
        );
    }
}

export class BlogPostView extends React.Component<any, State> {
    constructor(props: any) {
        super(props);

        this.state = { loadState: LoadState.Loading };
    }

    componentDidMount() {
        const userId = this.props.params.userId;
        const postId = this.props.params.postId;

        BlogPostStore.getPost(userId, postId,
            (post) => {
                this.setState({ post: post, loadState: LoadState.Finished });
            },
            (error) => {
                this.setState({ error: error, loadState: LoadState.Error });
            }
        );
    }

    render(): JSX.Element {
        switch (this.state.loadState) {
            case LoadState.Loading: return <p />;
            case LoadState.Error: return <p>Error: {this.state.error}</p>;
        }

        const post = this.state.post;
        if (post == null) {
            throw Error("Missing post");
        }
        const htmlContent = { __html: post.content };
        const tags = post.tags.map((t: string) => {
            const link = `/user/${post.ownerId}/tag/${t}`;
            return (<span key={t}><Link className="tag-link" to={link}>{t}</Link>&nbsp;</span>);
        });
        const date = post.createdOn;
        return (
            <div className="article-container">
                <article>
                    <header>
                        <h1>{post.title}</h1>
                    </header>
                    <div className="timestamp"><time dateTime={date.toISOString()}>{date.toLocaleDateString()}</time></div>
                    <div id="blog-content" dangerouslySetInnerHTML={htmlContent} />
                    <div className="tags">{tags}</div>
                </article>
            </div>);
    }
}