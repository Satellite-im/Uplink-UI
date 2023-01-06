use futures::channel::oneshot;
use warp::{crypto::DID, tesseract::Tesseract};

use crate::warp_runner::Account;

#[derive(Debug)]
pub enum MultiPassCmd {
    CreateIdentity {
        username: String,
        passphrase: String,
        rsp: oneshot::Sender<Result<(), warp::error::Error>>,
    },
    RequestFriend {
        did: DID,
        rsp: oneshot::Sender<Result<(), warp::error::Error>>,
    },
}

pub async fn handle_multipass_cmd(
    cmd: MultiPassCmd,
    tesseract: &mut Tesseract,
    account: &mut Account,
) {
    match cmd {
        MultiPassCmd::CreateIdentity {
            username,
            passphrase,
            rsp,
        } => {
            if let Err(e) = tesseract.unlock(passphrase.as_bytes()) {
                let _ = rsp.send(Err(e));
                return;
            }
            let r = match account.create_identity(Some(&username), None).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
            let _ = rsp.send(r);
        }
        MultiPassCmd::RequestFriend { did, rsp } => {
            let r = match account.send_request(&did).await {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            };
            let _ = rsp.send(r);
        }
    }
}
