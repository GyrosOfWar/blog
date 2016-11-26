import * as React from "react";

export class BlogPost {
    readonly title: string;
    readonly content: string;
    readonly id: number;
    readonly createdOn: Date;
    readonly ownerId: number;
    readonly tags: Array<string>;

    constructor (title: string, content: string, id: number, createdOn: Date, ownerId: number, tags: Array<string>) {
        this.title = title;
        this.content = content;
        this.id = id;
        this.createdOn = createdOn;
        this.ownerId = ownerId;
        this.tags = tags;
    }
}

export interface BlogPostProps {
    post: BlogPost
}

export class BlogPostView extends React.Component<BlogPostProps, {}> {
    render(): JSX.Element {
        const post = this.props.post;
        const htmlContent = {__html: post.content};
        return (
            <article>
                <h1>{post.title}</h1>
                <div id="blog-content" dangerouslySetInnerHTML={htmlContent} />
            </article>
        );
    }
}