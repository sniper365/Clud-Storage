export class Err<E> {
    private error: E;

    constructor(err: E) {
        this.error = err;
    }

    public deref(): E {
        return this.error;
    }
}

export default Err;
