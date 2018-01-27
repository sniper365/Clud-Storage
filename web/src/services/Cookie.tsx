class Cookie {
    constructor() {
        try {
            JSON.parse(document.cookie);
        } catch (_) {
            document.cookie = "{}";
        }
    }

    public get(property: string): string | object | undefined {
        return JSON.parse(document.cookie)[property];
    }

    public set(property: string, data: string | object) {
        const cookie = JSON.parse(document.cookie);

        cookie[property] = data;

        document.cookie = JSON.stringify(cookie);
    }

    public destroy() {
        document.cookie = "{}";
    }
}

export default new Cookie();
