import * as React from "react";

export class BlogPost {
    title: String
    content: String
    id: Number
    created_on: Date
    owner_id: Number
    tags: Array<String>
    published: boolean
}

export interface BlogPostProps {
    post: BlogPost
}