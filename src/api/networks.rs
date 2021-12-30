use crate::{api::ApiResource, models, opts, util::url, Result};

impl_api_ty!(
    Network => name
);

impl<'podman> Network<'podman> {
    api_doc! {
    Network => DeleteLibpod
    /// Delete this container. To delete this network forcefully use
    /// [`Network::force_delete`](Network::force_delete).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.networks().get("some-network").delete().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn delete(&self) -> Result<models::NetworkRmReport> {
        self.podman
            .delete_json(&format!("/libpod/networks/{}", &self.name))
            .await
    }}

    api_doc! {
    Network => DeleteLibpod
    /// Force remove this network removing associated containers. To delete network normally use
    /// [`Network::delete`](Network::delete).
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.networks().get("some-network").force_delete().await {
    ///     Ok(info) => println!("{:?}", info),
    ///     Err(e) => eprintln!("{}", e),
    /// }
    /// ```
    |
    pub async fn force_delete(&self) -> Result<models::NetworkRmReport> {
        let ep = url::construct_ep(
            &format!("/libpod/networks/{}", &self.name),
            Some(url::encoded_pair("force", true.to_string())),
        );
        self.podman.delete_json(&ep).await
    }}

    api_doc! {
    Network => ExistsLibpod
    /// Quick way to determine if a network exists by name or id.
    ///
    /// Examples:
    ///
    /// ```no_run
    /// let podman = Podman::unix("/run/user/1000/podman/podman.sock");
    ///
    /// match podman.networks().get("some-network").exists().await {
    ///     Ok(exists) => if exists {
    ///         println!("network exists!");
    ///     } else {
    ///         println!("network doesn't exists!");
    ///     },
    ///     Err(e) => eprintln!("check failed: {}", e),
    /// }
    /// ```
    |
    pub async fn exists(&self) -> Result<bool> {
        self.podman
            .resource_exists(ApiResource::Networks, &self.name)
            .await
    }}
}
