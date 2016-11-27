import * as React from "react";
import { login } from '../auth';

function getFromInput(id: string): string | null {
    const el = document.getElementById(id) as HTMLInputElement;
    return el.value;
}

export class Login extends React.Component<{}, {}> {
    onSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();
        const username = getFromInput('username-input');
        const password = getFromInput('password-input');
        if (username == null || password == null) {
            // TODO add error to state
            return;
        }
        login(username, password,
            token => {
                console.log(token);
            },
            error => {
                console.log(error);
            }
        );
    }

    render(): JSX.Element {
        return (
            <form className="form" onSubmit={this.onSubmit}>
                <h1>Login</h1>
                <div className="form-group">
                    <label className="form-label" htmlFor="username-input">Username</label>
                    <input type="text" id="username-input" className="text-input" />
                </div>
                <div className="form-group">
                    <label className="form-label" htmlFor="password-input">Password</label>
                    <input type="password" id="password-input" className="text-input" />
                </div>

                <div className="form-group">
                    <button type="submit" className="button" id="login-button">Login</button>
                </div>
            </form>
        )
    }
}