extern crate data_query;
extern crate regex;
extern crate kube_resource_extras;

pub mod constant;
pub mod kubernetes;
pub mod resource;
pub mod resource_collection;
pub mod resource_type;

use data_query::QueryError;
use kube::Error;
pub use kubernetes::ResourceQuery;

pub use resource_type::ResourceType;

pub mod prelude {
    pub use kube_resource_extras::*;
}

pub type KubeQueryResult<T> = Result<T, KubeQueryError>;

#[derive(Debug)]
pub enum KubeQueryError {
    QueryError(QueryError),
    RegexError(regex::Error),
    KubeError(kube::Error),
}

impl From<QueryError> for KubeQueryError {
    fn from(e: QueryError) -> Self {
        Self::QueryError(e)
    }
}

impl From<regex::Error> for KubeQueryError {
    fn from(e: regex::Error) -> Self {
        Self::RegexError(e)
    }
}

impl From<kube::Error> for KubeQueryError {
    fn from(e: Error) -> Self {
        Self::KubeError(e)
    }
}

#[cfg(test)]
mod test {
    use k8s_openapi::api::core::v1::Service;
    use kube::Api;
    use kube_core::params::ListParams;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    pub async fn simple_resource_access() {
        let client = kube::Client::try_default()
            .await
            .expect("Failed to create client");
        let api: Api<Service> = Api::all(client);
        for r in api
            .list(&ListParams::default().fields("metadata.name=kubernetes"))
            .await
            .map_err(|e| println!("{:?}", e))
            .expect("broke")
        {
            println!("{:?}", r);
        }
    }
}
