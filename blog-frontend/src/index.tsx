import * as React from "react";
import * as ReactDOM from "react-dom";
import { Router, Route, hashHistory, IndexRoute, Link } from 'react-router';

import { BlogPostView, BlogPost } from "./components/BlogPost";
import { Navigation } from "./components/Navigation";
import { Login } from "./components/Login";
import { logout } from "./auth";

import "../styles/main.scss";

const blogPost = new BlogPost("Test post", "Some content <p>New thing</p>", 0, new Date(), 0, ["video", "ruby", "cars"]);

class App extends React.Component<{}, {}> {
    render(): JSX.Element {
        return (
            <div id="main">
                <Navigation />
                {this.props.children}
            </div>
        )
    }
}

class Home extends React.Component<{}, {}> {
    render(): JSX.Element {
        return (
            <Link to="/user/0/post/1">Link</Link>
        );
    }
}

class Logout extends React.Component<{}, {}> {
    componentDidMount() {
        logout();
    }

    render(): JSX.Element {
        return <p>You were logged out!</p>
    }
}

ReactDOM.render(
    <Router history={hashHistory}>
        <Route path="/" component={App}>
            <IndexRoute component={Home} />
            <Route path="login" component={Login} />
            <Route path="logout" component={Logout} />
            <Route path="user/:userId/post/:postId" component={BlogPostView} />
        </Route>
    </Router>,
    document.getElementById("root")
);