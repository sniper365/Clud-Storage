class Cookie {
    constructor() {
        try {
            JSON.parse(document.cookie);
        } catch (_) {
            document.cookie = "{}";
        }
    }

    public get(property: string): string | object | undefined {
        let prop;

        try {
            prop = JSON.parse(document.cookie)[property];
        } catch (_) {
            this.destroy();
            prop = undefined;
        }

        return prop;
    }

    public set(property: string, data: string | object) {
        const cookie = JSON.parse(document.cookie);

        cookie[property] = data;

        document.cookie = JSON.stringify(cookie);
    }

    public destroy() {
        const cookies = document.cookie.split(";");

        for (const i of cookies) {
            const cookie = cookies[i];
            const eqPos = cookie.indexOf("=");
            const name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
            document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
        }
    }
}

export default new Cookie();
