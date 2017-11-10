import * as React from "react";
import { Route, Switch } from "react-router-dom";

import FolderList from "./folders/FolderList";
import FileList from "./files/FileList";

import Folder from "../models/Folder";

class Viewport extends React.Component<{}, { loaded: boolean, root?: Folder }> {
    constructor() {
        super();

        this.state = {
            loaded: false,
            root: undefined,
        };
    }

    public render() {
        return (
            <div id="viewport">
                <Switch>
                    <Route exact={true} path="/folders" component={() => (<FolderList root={undefined}/>)}/>
                    <Route path="/folders/:folder_id" component={({ match }) => (
                            <FolderList root={match.params.folder_id}/>)
                        }/>
                </Switch>

                <Switch>
                <Route exact={true} path="/folders" component={() => (<FileList root={undefined}/>)}/>
                <Route path="/folders/:folder_id" component={({ match }) => (
                        <FileList root={match.params.folder_id}/>)
                    }/>
                </Switch>
            </div>
        );
    }
}

export default Viewport;
