use std::path::Path;

use git2::{
    Error, FetchOptions, RemoteCallbacks,
    build::{CheckoutBuilder, RepoBuilder},
};

pub fn build(name: String) -> Result<(), Error> {
    match checkout(name) {
        Err(e) => return Err(e),
        Ok(_) => todo!(),
    }
}

//pub async fn build(name: String) {}
// git clone --no-checkout --depth=1 --filter=tree:0 git@github.com:leighmacdonald/smpkg-repo test-repo
// git sparse-checkout set --no-cone /connect
// git checkout
pub fn checkout(plugin_name: String) -> Result<(), Error> {
    let url = "https://github.com/leighmacdonald/projects/smpkgs-repo";
    let mut fo = FetchOptions::new();
    fo.depth(1);
    fo.remote_callbacks(RemoteCallbacks::new());

    let mut co = CheckoutBuilder::new();
    co.path(plugin_name);

    RepoBuilder::new()
        .branch("master")
        .with_checkout(co)
        .fetch_options(fo)
        .clone(url, Path::new("/projects/smpkgs-repo-remote"))?;
    Ok(())
}
