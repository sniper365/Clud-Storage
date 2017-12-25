import User from "../models/User";

import AuthPayload from "../responses/AuthPayload";

class AuthService {
    public getUser(): User {
        return JSON.parse(document.cookie).user;
    }

    public getToken(): string {
        return JSON.parse(document.cookie).token;
    }

    public authenticated(): boolean {
        try {
            return JSON.parse(document.cookie).token !== undefined;
        } catch (_) {
            return false;
        }
    }

    public authenticate( email: string, password: string ) {
        return fetch('api/login', {
            body: JSON.stringify( {
                'email': email,
                'password': password,
            } ),
            headers: {
                'Content-Type': 'application/json'
            },
            method: 'POST',
        }).then((response) => {
            return response.json();
        }).then((response: AuthPayload) => {
            if (response.success) {
                document.cookie = JSON.stringify({
                    token: response.token
                });
            }

            return response;
        });
    }

    public setUser(user_id: number) {
        return fetch('api/users/' + user_id, {
            headers: {
                'Authorization': 'Bearer ' + this.getToken(),
                'Content-Type': 'application/json',
            },
            method: 'GET',
        }).then((response) => {
            return response.json();
        }).then((response: User) => {
            const cookie = JSON.parse(document.cookie);

            cookie.user = response;

            document.cookie = JSON.stringify(cookie);
        });
    }
}

export default new AuthService();
