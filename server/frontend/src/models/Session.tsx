import User from "./User";

export class Session {
    public user_id: number;
    public token: string;
    public user: User;
}

export default Session;
