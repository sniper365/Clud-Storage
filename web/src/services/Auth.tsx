import * as $ from "jquery";

import User from "../models/User";

import AuthPayload from "../responses/AuthPayload";

class AuthService {
    public isAuthenticated: boolean = false;

    public login( email: string, password: string ): boolean {
        let payload: AuthPayload = {
            token: '',
            user_id: 0
        };

        $.ajax("/api/login", {
            async: false,
            data: JSON.stringify( {
                'email': email,
                'password': password,
            } ),
            dataType: 'json',
            headers: {
                'Content-Type': 'application/json'
            },
            method: 'POST',
            success: ( response: AuthPayload, _status: string ) => {
                payload = response;

                $('[name="_token"]').attr( 'content', payload.token );

                this.isAuthenticated = true;
            },
        });

        if ( this.isAuthenticated ) {
            $.ajax("api/users/" + payload.user_id, {
                async: false,
                dataType: 'json',
                headers: {
                    'Authorization': 'Bearer ' + payload.token,
                    'Content-Type': 'application/json',
                },
                method: 'GET',
                success: ( response: object, _status: string ) => {
                    $('[name="_user"]').attr( 'content', JSON.stringify(response) );
                },
            });
        }

        return this.isAuthenticated;
    }

    public getUser(): User {
        const user: string | undefined = $('[name="_user"]').attr( 'content' );

        if ( user !== undefined ) {
            return JSON.parse( user );
        }

        return {
            email: '',
            name: '',
            user_id: 0,
        };
    }

    public getToken(): string {
        const token = $('[name="_token"]').attr( 'content' );

        if ( token !== undefined ) {
            return token;
        }

        return "";
    }
}

export default new AuthService();
