use bytes::Bytes;

use crate::{Error, Result};
use crate::cache;
use crate::config::{self, Source};

mod github;
mod http;
mod path;

pub async fn load<'a>(cache: cache::Entry<'a>, source: &config::Source, transform: &config::Transform) -> Result<cache::Reference> {
    match source {
        Source::GitHubArtifacts { github, workflow, branch, artifact } => {
            match github.split("/").collect::<Vec<&str>>().as_slice() {
                [owner, repository] => {
                    let filter = github::Filter {
                        workflow: workflow.clone(),
                        branch: branch.clone(),
                        artifact: artifact.clone(),
                    };

                    github::load(cache, owner, repository, filter, transform).await
                },
                _ => Err(Error::MalformedGitHubReference(github.clone())),
            }
        }
        Source::Url { url } => http::load(cache, url, transform).await,
        Source::Path { path } => path::load(cache, path, transform).await,
    }
}

pub struct File {
    pub name: String,
    pub bytes: Bytes,
}
