use chrono::Utc;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::StatusClass;
use rocket::{Data, Request, Response, Rocket};

// While not actually throwing errors,
//  this is a core logging functionality that,
// in some but not all instances, force
pub struct LogFairing;

impl Fairing for LogFairing {
    fn info(&self) -> Info {
        Info {
            name: "Request/Response Logging",
            kind: Kind::Request | Kind::Response | Kind::Launch,
        }
    }

    fn on_launch(&self, _r: &Rocket) {
        log!(
            "fatal",
            "Application launching, current time: {}",
            Utc::now()
        );
    }

    fn on_request(&self, request: &mut Request, _d: &Data) {
        match request.client_ip() {
            Some(ip) => {
                log!("fatal", "Got request from IP {}", ip);
            }
            None => {
                log!("fatal", "Got request from unknown IP");
            }
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if response.status().class() != StatusClass::ServerError {
            return;
        }

        log!(
            "fatal",
            "Server Error Thrown, info: \n\tStatus {}\n\tAction {}",
            response.status(),
            request
        );
    }
}
