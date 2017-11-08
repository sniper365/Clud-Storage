import * as $ from "jquery";
import * as React from "react";
import { Route, Switch } from "react-router-dom";
import AuthService from "../services/Auth";
import FolderList from "./FolderList";

import Folder from "../models/Folder";

class Viewport extends React.Component<{}, { loaded: boolean, root?: Folder }> {
    constructor() {
        super();

        this.state = {
            loaded: false,
            root: undefined,
        };

        this.load = this.load.bind(this);
    }

    public componentDidMount() {
        this.load();
    }

    public load(): void {
        const user = AuthService.getUser();

        const path = ( !this.state.loaded )
            ? "api/users/" + user.user_id + "/root"
            : "api/users/" + user.user_id + "/folders/" + this.state.root;

        $.ajax(path, {
            dataType: 'json',
            headers: {
                'Authorization': 'Bearer ' + AuthService.getToken(),
                'Content-Type': 'application/json',
            },
            method: 'GET',
            success: ( response: Folder, _status: string ) => {
                this.setState({
                    loaded: true,
                    root: response
                });
            },
        });
    }

    public render() {
        const root = this.state.root;

        return (
            <div id="viewport">
                <Switch>
                    <Route path="/folders" component={() => (<FolderList root={root}/>)}/>
                    <Route path="/folders/:folder_id" component={FolderList} />
                </Switch>
            </div>
        );
    }
}

export default Viewport;
