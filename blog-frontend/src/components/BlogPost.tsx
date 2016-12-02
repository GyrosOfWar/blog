import * as React from "react";
import { Link } from "react-router";
import { BlogPostStore } from "../store/BlogPostStore";

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
    error?: string
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
        this.state = {};
    }

    fetchPost(): void {
        console.log("fetchPost called");
        BlogPostStore.getPost(this.props.params.userId, this.props.params.postId,
            (post) => {
                this.setState({ post: post });
                console.log("Set state");
            },
            (e, xhr, resp) => {
                this.setState({ error: resp.error.description })
            }
        );
    }

    componentWillMount(): void {
        console.log("componentWillMount called");
        this.fetchPost();
    }

    render(): JSX.Element {
        console.log("Render called");
        if (this.state.error) {
            return <span>Error: {this.state.error}</span>;
        }

        if (!this.state.post) {
            console.log("No post found, showing loading");
            return <Loading />
        }

        const post = this.state.post;
        const htmlContent = { __html: post.content };
        const tags = <p></p>;
        // const tags = post.tags.map((t: string) => {
        //     const link = `/user/${post.ownerId}/tag/${t}`;
        //     return (<span key={t}><Link className="tag-link" to={link}>{t}</Link>&nbsp;</span>);
        // });
        const el = (
            <article>
                <header>
                    <h1>{post.title}</h1>
                </header>
                <p>Posted on <time>{post.createdOn}</time></p>
                <div id="blog-content" dangerouslySetInnerHTML={htmlContent} />
                <div className="tags">{tags}</div>
            </article>
        );

        return el;
    }
}