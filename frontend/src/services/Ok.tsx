export class Ok<T> {
    private ok: T;

    constructor(ok: T) {
        this.ok = ok;
    }

    public deref(): T {
        return this.ok;
    }
}

export default Ok;
