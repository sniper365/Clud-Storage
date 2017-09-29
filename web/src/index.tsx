import * as React from "react";
import * as ReactDOM from "react-dom";
import LoginForm from "./objects/LoginForm";
import registerServiceWorker from "./registerServiceWorker";

import "./index.css";

ReactDOM.render(<LoginForm />, document.getElementById("root") as HTMLElement);
registerServiceWorker();
