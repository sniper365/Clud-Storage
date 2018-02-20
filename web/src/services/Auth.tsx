import User from "../models/User";

import TokenService from "./Token";

class AuthService {
    private auth_user: User;

    public authenticated(): boolean {
        return (typeof TokenService.getToken() === 'string') ? true : false;
    }

    public user(): Promise<User> {
        const token = TokenService.payload();

        if ( typeof this.auth_user !== 'undefined' ) {
            return Promise.resolve(this.auth_user);
        }

        if ( typeof token === 'undefined' ) {
            return Promise.reject("No session found");
        }

        return fetch( "/api/users/" + token.user_id, {
            headers: {
                'Authorization': 'Bearer ' + TokenService.getToken(),
            }
        }).then((response) => {
            return response.json();
        }).then((user: User) => {
            this.auth_user = user;

            return user;
        });
    }

    public logout() {
        TokenService.setToken("");
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
        }).then((response) => {
            if(response['token'] && response['user']) {
                TokenService.setToken(response['token']);

                this.auth_user = response['user'];
            }

            return response;
        });
    }
}

export default new AuthService();
