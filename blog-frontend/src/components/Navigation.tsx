import * as React from "react";
import { Link } from 'react-router';
import { isLoggedIn } from '../auth'

export class Navigation extends React.Component<{}, {}> {
    render(): JSX.Element {
        let log: JSX.Element;
        if (isLoggedIn()){
            log = <Link to="/logout">Logout</Link>;
        } else {
            log = <Link to="/login">Login</Link>;
        }
        return (
            <nav>
                <ul>
                    <li><Link to="/">Home</Link></li>
                    <li>{log}</li>
                </ul>
            </nav>
        )
    }
}