import { logout } from '../auth';
import * as React from 'react';

export class Logout extends React.Component<{}, {}> {
    componentDidMount() {
        logout();
    }

    render(): JSX.Element {
        return <p>You were logged out!</p>
    }
}