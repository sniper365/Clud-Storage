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
        } catch(_) {
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
        var cookies = document.cookie.split(";");

        for (var i = 0; i < cookies.length; i++) {
            var cookie = cookies[i];
            var eqPos = cookie.indexOf("=");
            var name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;
            document.cookie = name + "=;expires=Thu, 01 Jan 1970 00:00:00 GMT";
        }
    }
}

export default new Cookie();
