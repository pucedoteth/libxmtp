//! Different ways to create a [`crate::DbgClient`]
use super::*;
use crate::app::types::*;

pub async fn new_registered_client(
    network: args::BackendOpts,
    wallet: Option<&types::EthereumWallet>,
) -> Result<crate::DbgClient> {
    let local_wallet = if let Some(w) = wallet {
        w.clone().into_ethers()
    } else {
        generate_wallet().into_ethers()
    };
    new_client_inner(network, &local_wallet, None).await
}

/// Create a new client + Identity
pub async fn temp_client(
    network: &args::BackendOpts,
    wallet: Option<&types::EthereumWallet>,
) -> Result<crate::DbgClient> {
    let local_wallet = if let Some(w) = wallet {
        w.clone().into_ethers()
    } else {
        generate_wallet().into_ethers()
    };

    let tmp_dir = (*crate::constants::TMPDIR).path();
    let public = hex::encode(local_wallet.get_address());
    let name = format!("{public}:{}.db3", u64::from(network));

    new_client_inner(
        network.clone(),
        &local_wallet,
        Some(tmp_dir.to_path_buf().join(name)),
    )
    .await
}

pub async fn client_from_identity(
    identity: &Identity,
    network: &args::BackendOpts,
) -> Result<crate::DbgClient> {
    let path = identity.db_path(network)?;
    debug!(
        inbox_id = hex::encode(identity.inbox_id),
        db_path = %path.display(),
        "creating client from identity"
    );
    existing_client_inner(network, path).await
}

/// Create a new client + Identity & register it
async fn new_client_inner(
    network: args::BackendOpts,
    wallet: &LocalWallet,
    db_path: Option<PathBuf>,
) -> Result<crate::DbgClient> {
    let url = url::Url::from(network.clone());
    let is_secure = url.scheme() == "https";
    trace!(url = %url, is_secure, "create grpc");
    let api = crate::GrpcClient::create(url.as_str().to_string(), is_secure).await?;

    let nonce = 1;
    let inbox_id = generate_inbox_id(&wallet.get_address(), &nonce).unwrap();

    let dir = if let Some(p) = db_path {
        p
    } else {
        let dir = crate::app::App::db_directory(&network)?;
        let db_name = format!("{inbox_id}:{}.db3", u64::from(network));
        dir.join(db_name)
    };

    let client = crate::DbgClient::builder(IdentityStrategy::CreateIfNotFound(
        inbox_id,
        wallet.get_address(),
        nonce,
        None,
    ))
    .api_client(api)
    .store(
        EncryptedMessageStore::new(
            StorageOption::Persistent(dir.into_os_string().into_string().unwrap()),
            [0u8; 32],
        )
        .await?,
    )
    .build()
    .await?;

    register_client(&client, wallet).await?;

    Ok(client)
}

pub async fn register_client(client: &crate::DbgClient, owner: impl InboxOwner) -> Result<()> {
    let signature_request = client.context().signature_request();

    trace!(
        inbox_id = client.inbox_id(),
        address = owner.get_address(),
        installation_id = hex::encode(client.installation_public_key()),
        "registering client"
    );
    if let Some(mut req) = signature_request {
        let signature_text = req.signature_text();
        let unverified_signature = UnverifiedSignature::RecoverableEcdsa(
            UnverifiedRecoverableEcdsaSignature::new(owner.sign(&signature_text)?.into()),
        );
        req.add_signature(unverified_signature, client.scw_verifier())
            .await?;

        client.register_identity(req).await?;
    } else {
        warn!(address = owner.get_address(), "Signature request empty!");
    }

    Ok(())
}

/// Create a new client + Identity
async fn existing_client_inner(
    network: &args::BackendOpts,
    db_path: PathBuf,
) -> Result<crate::DbgClient> {
    let url = url::Url::from(network.clone());
    let is_secure = url.scheme() == "https";
    let api = crate::GrpcClient::create(url.as_str().to_string(), is_secure).await?;

    let store = EncryptedMessageStore::new(
        StorageOption::Persistent(db_path.clone().into_os_string().into_string().unwrap()),
        [0u8; 32],
    )
    .await;
    if let Err(e) = &store {
        error!(db_path = %(&db_path.as_path().display()), "{e}");
    }
    let client = crate::DbgClient::builder(IdentityStrategy::CachedOnly)
        .api_client(api)
        .store(store?)
        .build()
        .await?;

    Ok(client)
}
