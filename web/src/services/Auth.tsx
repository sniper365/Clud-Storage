import User from "../models/User";

import AuthPayload from "../responses/AuthPayload";

class AuthService {
    public isAuthenticated: boolean = false;

    private user: User;

    private token: string;

    public getUser(): User {
        return this.user;
    }

    public getToken(): string {
        return this.token;
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
                this.token = response.token;

                this.isAuthenticated = true;
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
            this.user = response;
        });
    }
}

export default new AuthService();
