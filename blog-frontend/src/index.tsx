import * as React from "react";
import * as ReactDOM from "react-dom";

import {BlogPostView, BlogPost} from "./components/BlogPost";

const blogPost = new BlogPost("Test post", "Some content <p>New thing</p>", 0, new Date(), 0, ["tag"]);

ReactDOM.render(
    <BlogPostView post={blogPost} />,
    document.getElementById("example")
);