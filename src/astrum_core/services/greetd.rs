use std::process::ExitCode;
use std::{clone, env};
use std::{borrow::Borrow, collections::HashSet, os::unix::net::UnixStream, path::PathBuf, process::Command};

use lazy_static::lazy_static;
use greetd_ipc::codec::SyncCodec;
use greetd_ipc::Request;
use greetd_ipc::Response;
use greetd_ipc::AuthMessageType;
use greetd_ipc::ErrorType;
use log::info;

// lazy_static! {
//     pub static ref GREETD_SESSIONS: Arc<Mutex<HashMap<String, Greetd>>> = Arc::new(Mutex::new(HashMap::new()));
// }

// this will create a login session, if one already exists and if successfull will attempt to log in
// so store all current sessions in a thingalang
pub fn greetd_log_in(username: String, attempt: String, command: String) -> String {
    info!("Got greetd login request: user: {}  attempt: {} command: {}", username, attempt, command);
    let mut stream = UnixStream::connect(env::var("GREETD_SOCK").expect("Couldnt find GREETD_SOCK")).expect("Couldnt connect to GREETD_SOCK");

    let mut session = Request::CreateSession { username };


    let mut starting = false;
    let mut success = false;
    loop {
        if success {
            std::process::exit(0);
        }
        session.write_to(&mut stream);

        info!("looping greetd");
        match Response::read_from(&mut stream).expect("Response couldnt connect to greetd stream") {
            Response::AuthMessage { auth_message_type, auth_message } => {
                info!("attempting auth message {:?}", auth_message_type);
                let response = match auth_message_type {
                    AuthMessageType::Visible => Some(attempt.clone()),
                    AuthMessageType::Secret => Some(attempt.clone()),
                    AuthMessageType::Info => {
                        eprintln!("greetd info: {}", auth_message);
                        None
                    },
                    AuthMessageType::Error => {
                        eprintln!("greetd error: {}", auth_message);
                        None
                    }
                };

                session = Request::PostAuthMessageResponse { response };
            },
            Response::Success => {
                info!("!!!!!!!!!!!!!! greetd success! {} and starting: {}",  success, starting);
                if starting {
                    success = true;
                    return "login_success".to_string();
                } else {
                    starting = true;

                    session = Request::StartSession { cmd: vec![ command.to_string() ], env: vec![] };
                }
            },
            Response::Error { error_type, description } => {
                info!("failure.......................");
                Request::CancelSession.write_to(&mut stream).unwrap();

                match error_type {
                    ErrorType::AuthError => return "login_failure".to_string(),
                    ErrorType::Error => {
                        eprintln!("greetd login error: {}", description);
                        return "login_failure".to_string()
                    }
                }
            }
        }
    }
}
