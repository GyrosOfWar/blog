import * as React from "react";
import * as ReactDOM from "react-dom";

import {BlogPostView, BlogPost} from "./components/BlogPost";
import {Navigation} from "./components/Navigation";

const blogPost = new BlogPost("Test post", "Some content <p>New thing</p>", 0, new Date(), 0, ["tag"]);

class App extends React.Component<{}, {}> {
    render(): JSX.Element {
        return (
            <div id="main">
                <Navigation />
                <BlogPostView post={blogPost} />
            </div>
        )
    }
}

ReactDOM.render(
    <App />,
    document.getElementById("root")
);