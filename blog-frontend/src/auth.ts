import * as jwtDecode from 'jwt-decode';
import * as qwest from 'qwest';

const TOKEN_KEY = 'jwt';

export class Token {
    readonly rawToken: string;
    readonly data: any;

    constructor(token: string) {
        this.rawToken = token;
        this.data = jwtDecode(token);
    }

    toJSON(): string {
        return JSON.stringify(this.rawToken);
    }

    static fromJSON(json: string | null): Token | null {
        if (json == null) {
            return null;
        }
        const raw = JSON.parse(json);
        return new Token(raw);
    }
}

export function isLoggedIn(): boolean {
    return localStorage.getItem(TOKEN_KEY) != null;
}

export function getToken(): Token | null {
    return Token.fromJSON(localStorage.getItem(TOKEN_KEY));
}

export function login(username: string, password: string, successCallback: (t: Token) => any, errorCallback: (e: any, xhr?: any, response?: any) => any) {
    const info = { name: username, password: password };
    qwest.post('/api/token', info, {dataType: 'json', responseType: 'json'})
        .then((xhr, response) => {
            console.log(response);
            const raw = response.result;

            const token = new Token(raw);
            localStorage.setItem(TOKEN_KEY, token.toJSON());
            successCallback(token);
        })
        .catch((err, xhr, response) => {
            errorCallback(err, xhr, response);
        }
    );
}

export function logout() {
    localStorage.removeItem(TOKEN_KEY);
}