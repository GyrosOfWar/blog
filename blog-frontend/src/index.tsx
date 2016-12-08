import * as React from "react";
import * as ReactDOM from "react-dom";
import { Router, Route, hashHistory, IndexRoute, Link } from 'react-router';

import { BlogPostView } from "./components/BlogPost";
import { Navigation } from "./components/Navigation";
import { Login } from "./components/Login";
import { Logout } from "./components/Logout";
import { BlogEditor }  from "./components/Editor";

import "../styles/main.scss";

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

const router = <Router history={hashHistory}>
        <Route path="/" component={App}>
            <IndexRoute component={Home} />
            <Route path="login" component={Login} />
            <Route path="logout" component={Logout} />
            <Route path="user/:userId/post/:postId" component={BlogPostView} />
            <Route path="user/:userId/post/:postId/edit" component={BlogEditor} />
        </Route>
    </Router>;

ReactDOM.render(
    router,
    document.getElementById("root")
);