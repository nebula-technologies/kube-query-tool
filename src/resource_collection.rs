use crate::resource::Resource;
use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::batch::v1::CronJob;
use k8s_openapi::api::core::v1::{Namespace, Pod, Service};
use k8s_openapi::api::networking::v1::Ingress;
use kube_core::DynamicObject;
use kube_resource_extras::istio::{DestinationRule, EnvoyFilter, Gateway, VirtualService};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ResourceCollection {
    inner: Vec<Resource>,
}

impl ResourceCollection {
    pub fn concat(&mut self, rc: ResourceCollection) {
        let inner = self.inner.clone();
        self.inner = vec![inner, rc.inner].concat();
    }
}

impl Deref for ResourceCollection {
    type Target = Vec<Resource>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ResourceCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl Default for ResourceCollection {
    fn default() -> Self {
        ResourceCollection { inner: Vec::new() }
    }
}

impl From<Vec<Gateway>> for ResourceCollection {
    fn from(v: Vec<Gateway>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}
impl From<Vec<VirtualService>> for ResourceCollection {
    fn from(v: Vec<VirtualService>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}
impl From<Vec<DestinationRule>> for ResourceCollection {
    fn from(v: Vec<DestinationRule>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}
impl From<Vec<EnvoyFilter>> for ResourceCollection {
    fn from(v: Vec<EnvoyFilter>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}
impl From<Vec<Service>> for ResourceCollection {
    fn from(v: Vec<Service>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}

impl From<Vec<Ingress>> for ResourceCollection {
    fn from(v: Vec<Ingress>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}

impl From<Vec<Namespace>> for ResourceCollection {
    fn from(v: Vec<Namespace>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}

impl From<Vec<Pod>> for ResourceCollection {
    fn from(v: Vec<Pod>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}

impl From<Vec<Deployment>> for ResourceCollection {
    fn from(v: Vec<Deployment>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}

impl From<Vec<CronJob>> for ResourceCollection {
    fn from(v: Vec<CronJob>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}

impl From<Vec<DynamicObject>> for ResourceCollection {
    fn from(v: Vec<DynamicObject>) -> Self {
        v.into_iter()
            .map(|t| t.into())
            .collect::<Vec<Resource>>()
            .into()
    }
}

impl From<Vec<Resource>> for ResourceCollection {
    fn from(v: Vec<Resource>) -> Self {
        ResourceCollection { inner: v }
    }
}
