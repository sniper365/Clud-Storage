import * as React from "react";

import AuthService from "../../services/Auth";
import TokenService from "../../services/Token";

import ErrorModel from "../../models/Error";
import { Folder as FolderModel } from "../../models/Folder";

import { Panel, PanelBody, PanelHeader } from "../../components/utils/Panel";
import {
    Button,
    Form,
    FormGroup,
    Input,
    Label,
} from 'reactstrap';

interface Props {
    root: number;
    on_save?: (response: FolderModel) => void;
    on_error?: (response: ErrorModel) => void;
}

class NewFolderForm extends React.Component<Props, { name: string, pending: boolean }> {
    constructor() {
        super();

        this.state = {
            name: '',
            pending: false
        };

        this.set_name = this.set_name.bind(this);
        this.save = this.save.bind(this);
    }

    public save(e) {
        e.preventDefault();

        this.setState({
            pending: true,
        });

        AuthService.user().then((user) => {
            const path = "/api/users/" + user.user_id + "/folders/";

            fetch(path, {
                method: 'post',
                headers: {
                    'Authorization': 'Bearer ' + TokenService.getToken(),
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    name: this.state.name,
                    parent_id: Number(this.props.root),
                }),
            }).then((response) => {
                return response.json();
            }).then((response) => {
                if (response.status_code >= 400) {
                    this.props.on_error && this.props.on_error(response);
                } else {
                    this.props.on_save && this.props.on_save(response);
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
