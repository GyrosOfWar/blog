import * as React from "react";
import * as Router from 'react-router';

export class Navigation extends React.Component<{}, {}> {
    render(): JSX.Element {
        return (
            <nav>
                <ul className="nav-list">
                    <li className="nav-item">
                        <Router.Link to="/">Home</Router.Link>
                    </li>
                </ul>
            </nav>
        )
    }
}