use crate::constant::{ANNOTATION_QUERY, LABEL_QUERY};

use crate::resource_collection::ResourceCollection;
use crate::{constant, KubeQueryError, KubeQueryResult, ResourceType};
use data_query::query;
use k8s_openapi::api::{
    apps::v1::Deployment,
    batch::v1::CronJob,
    core::v1::{Namespace, Pod, Service},
    networking::v1::Ingress,
};
use kube::Api;
use kube_core::params::ListParams;
use kube_core::ResourceExt;
use kube_resource_extras::istio::{DestinationRule, EnvoyFilter, Gateway, VirtualService};
use railsgun::{BlockInPlaceResult, FutureResult, Merge};
use regex::Regex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

use std::fmt::Debug;
use std::future::Future;

pub fn k8s_client() -> impl Future<Output = Result<kube::Client, kube::error::Error>> {
    kube::Client::try_default()
}

#[derive(Debug, Clone)]
pub struct FilterMatch {
    key: PathMatch,
    value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PathMatch {
    Namespace,
    Annotation,
    Label,
    Query(String),
    None,
}
impl From<&str> for PathMatch {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "annotation" => Self::Annotation,
            "namespace" => Self::Namespace,
            "label" => Self::Label,
            "" => Self::None,
            _ => Self::Query(s.to_string()),
        }
    }
}

impl ToString for PathMatch {
    fn to_string(&self) -> String {
        match self {
            PathMatch::Namespace => constant::NAMESPACE_QUERY,
            PathMatch::Annotation => constant::ANNOTATION_QUERY,
            PathMatch::Label => constant::LABEL_QUERY,
            PathMatch::Query(q) => q,
            PathMatch::None => "",
        }
        .to_string()
    }
}

#[derive(Clone)]
pub struct AnnotationFilter {
    key: String,
    value: Option<String>,
}

#[derive(Clone)]
pub struct ResourceQuery {
    resource: Vec<ResourceType>,
    filter: Vec<FilterMatch>,
    namespace: Option<String>,
    name: Option<String>,
}

impl ResourceQuery {
    pub fn select<T: Into<ResourceType>>(&mut self, resource: Vec<T>) -> &mut Self {
        self.resource = resource
            .into_iter()
            .map(|t| t.into())
            .collect::<Vec<ResourceType>>();
        self
    }

    pub fn filter(&mut self, key: Option<String>, value: Option<String>) -> &mut Self {
        let mut filter = FilterMatch {
            key: match key {
                None => PathMatch::None,
                Some(s) => PathMatch::from(s.as_str()),
            },
            value,
        };
        match &filter.key {
            PathMatch::Namespace => {
                if let Some(ns) = &filter.value {
                    self.namespace = Some(ns.clone())
                }
            }
            PathMatch::Annotation => filter.key = PathMatch::Query(ANNOTATION_QUERY.to_string()),
            PathMatch::Label => filter.key = PathMatch::Query(LABEL_QUERY.to_string()),
            _ => {}
        }
        self.filter.push(filter);
        self
    }

    pub async fn query(&self) -> KubeQueryResult<ResourceCollection> {
        let mut resources = ResourceCollection::default();
        let client = self.get_client().await.map_err(KubeQueryError::from)?;
        for resource in &self.resource {
            resources.concat(match resource {
                ResourceType::Gateway => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<Gateway>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::VirtualService => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<VirtualService>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::DestinationRule => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<DestinationRule>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::EnvoyFilter => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<EnvoyFilter>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::Service => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<Service>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::Ingress => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<Ingress>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::Namespace => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<Namespace>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::Pod => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<Pod>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::Deployment => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<Deployment>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                ResourceType::CronJob => (self.get_resource_with(&client).await
                    as KubeQueryResult<Vec<CronJob>>)
                    .map(|t| t.into())
                    .unwrap_or_default(),
                // ResourceType::DynamicObject => (self.get_resource_with(&client).await as Result<Vec<DynamicObject>, ()>).map(|t| t.into()).unwrap_or(Default::default()),
                _ => Default::default(),
            })
        }
        Ok(resources)
    }

    fn get_client(&self) -> impl Future<Output = Result<kube::Client, kube::error::Error>> {
        k8s_client()
    }

    async fn get_resource_with<K: kube::Resource + Clone + Debug + DeserializeOwned + Serialize>(
        &self,
        c: &kube::Client,
    ) -> KubeQueryResult<Vec<K>>
    where
        <K as kube::Resource>::DynamicType: Default + Clone,
    {
        let mut resources = Vec::new();
        let api: Api<K> = if let Some(ns) = &self.namespace {
            Api::namespaced(c.clone(), &ns)
        } else {
            Api::all(c.clone())
        };

        let mut list_params = ListParams::default();

        if let Some(name) = &self.name {
            let field = format!("metadata.name={}", name);
            list_params = list_params.fields(field.as_str());
        }

        let api_results = api.list(&list_params).await.map_err(KubeQueryError::from)?;

        for p in api_results {
            if self
                .filter
                .iter()
                .map(|filter| {
                    query_match(
                        serde_json::to_value(&p).unwrap(),
                        filter.clone().key.to_string(),
                        filter.clone().value,
                    )
                })
                .all(|b| b)
            {
                resources.push(p)
            }
        }
        Ok(resources)
    }
}

fn query_match(d: Value, q: String, rmatch: Option<String>) -> bool {
    if let Some(s) = rmatch {
        if s.starts_with("//") && s.ends_with("//") {
            let regex_res = Regex::new(&s).map_err(|e| KubeQueryError::from(e));
            query(d, q.as_str())
                .map_err(|e| KubeQueryError::from(e))
                .merge(regex_res, |t, r| {
                    Ok(match t {
                        Value::String(tt) => *tt == s,
                        Value::Array(a) => a
                            .iter()
                            .map(|t| match t {
                                Value::String(t) => *t == s,
                                _ => t.to_string() == s,
                            })
                            .any(|b| b),
                        _ => false,
                    })
                })
                .unwrap_or(false)
        } else {
            query(d, q.as_str())
                .map(|t| match t {
                    Value::String(tt) => *tt == s,
                    Value::Array(a) => a
                        .iter()
                        .map(|t| match t {
                            Value::String(t) => *t == s,
                            _ => t.to_string() == s,
                        })
                        .any(|b| b),
                    _ => false,
                })
                .unwrap_or(false)
        }
    } else {
        query(d, q.as_str())
            .map(|t| match t {
                Value::Array(a) => !a.is_empty(),
                Value::Object(m) => !m.is_empty(),
                Value::Null => false,
                _ => true,
            })
            .unwrap_or(false)
    }
}

impl Default for ResourceQuery {
    fn default() -> Self {
        use ResourceType::*;
        ResourceQuery {
            resource: vec![
                Gateway,
                VirtualService,
                Service,
                Ingress,
                Namespace,
                Pod,
                Deployment,
                CronJob,
            ],
            filter: Vec::new(),
            namespace: None,
            name: None,
        }
    }
}

#[cfg(test)]
mod test {

    //
    // #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    // pub async fn simple_resource_access() {
    //     crate::ResourceQuery::default().query().await;
    // }
    // #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    // pub async fn simple_get_virtualservice() {
    //     crate::ResourceQuery::default()
    //         .get(vec![ResourceType::VirtualService])
    //         .query()
    //         .await;
    // }
    // #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    // pub async fn simple_get_gateway() {
    //     crate::ResourceQuery::default()
    //         .get(vec![ResourceType::Gateway])
    //         .query()
    //         .await;
    // }

    use crate::ResourceQuery;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    pub async fn test_query_kubernetes() {
        let mut query = ResourceQuery::default();
        let value = query
            .filter(Some("Namespace".to_string()), Some("wordpress".to_string()))
            .query()
            .await;

        println!("{:?}", value.unwrap().len());
    }
}
