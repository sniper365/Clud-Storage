import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import ErrorModel from "../../models/Error";
import { Folder as FolderModel } from "../../models/Folder";

import { Button, Form, FormGroup, Input, Label } from 'reactstrap';
import { Panel, PanelBody, PanelHeader } from "../../components/utils/Panel";

interface Props {
    root: number;
    on_save?: (response: FolderModel) => void;
    on_error?: (response: ErrorModel) => void;
}

interface State {
    name: string;
    pending: boolean;
}

class NewFolderForm extends React.Component<Props, State> {
    constructor() {
        super();

        this.state = {
            name: '',
            pending: false
        };

        this.set_name = this.set_name.bind(this);
        this.save = this.save.bind(this);
    }

    public save(e: React.MouseEvent<HTMLButtonElement>) {
        e.preventDefault();

        this.setState({
            pending: true,
        });

        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/folders/";

            fetch(path, {
                body: JSON.stringify({
                    name: this.state.name,
                    parent_id: Number(this.props.root),
                }),
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                },
                method: 'post',
            }).then((response) => {
                return response.json();
            }).then((response) => {
                if (response.status_code >= 400) {
                    if (this.props.on_error) { this.props.on_error(response); }
                } else {
                    if (this.props.on_save) { this.props.on_save(response); }
                }
            });
        });
    }

    public render() {
        return (
            <Panel>
                <PanelHeader>
                    Create New Folder
                </PanelHeader>

                <PanelBody>
                    <Form>
                        <FormGroup>
                            <Label htmlFor="name">Name</Label>
                            <Input id="name" type="text" className="input" onChange={this.set_name}/>
                        </FormGroup>

                        <Button className="button button-primary float-right" onClick={this.save} type="submit">
                            {this.state.pending ? 'Saving...' : 'Save'}
                        </Button>

                        <div className="clearfix"/>
                    </Form>
                </PanelBody>
            </Panel>
        );
    }

    private set_name(e: React.ChangeEvent<HTMLInputElement>) {
        this.setState({
            name: e.target.value,
        });
    }
}

export default NewFolderForm;
