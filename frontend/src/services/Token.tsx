import Cookie from "./Cookie";

import Token from "../models/Token";

class TokenService {
    public getToken(): string | undefined {
        const token = Cookie.get('token');

        if (typeof token === 'string'  && token.length > 0) {
            return token;
        }

        return undefined;
    }

    public setToken(token: string) {
        Cookie.set('token', token);
    }

    public payload(): Token | undefined {
        const token = this.getToken();

        if (typeof token === 'undefined') {
            return undefined;
        }

        const payload = token.split('.')[1];

        return JSON.parse(atob(payload));
    }
}

export default new TokenService();
