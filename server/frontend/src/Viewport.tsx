import * as React from "react";
import { Route, Switch } from "react-router-dom";

import FolderView from "./views/Folder";
import HomeView from "./views/Home";
import SettingsView from "./views/Settings";

class Viewport extends React.Component<{}, { }> {
    constructor() {
        super();
    }

    public render() {
        return (
            <div id="viewport" className="container-fluid">
                <Switch>
                    <Route path="/folders/:folder_id" component={({ match }) => (
                        <FolderView root={match.params.folder_id}/>
                    )}/>

                    <Route path="/settings" component={() => (
                        <SettingsView/>
                    )}/>

                    <Route path="" component={() => (
                        <HomeView/>
                    )}/>
                </Switch>
            </div>
        );
    }
}

export default Viewport;
