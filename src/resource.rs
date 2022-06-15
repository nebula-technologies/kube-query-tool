use k8s_openapi::api::apps::v1::Deployment;
use k8s_openapi::api::batch::v1::CronJob;
use k8s_openapi::api::core::v1::{Namespace, Pod, Service};
use k8s_openapi::api::networking::v1::Ingress;
use kube_core::DynamicObject;
use kube_resource_extras::istio::{DestinationRule, EnvoyFilter, Gateway, VirtualService};

#[derive(Clone, Debug)]
pub enum Resource {
    Gateway(Gateway),
    VirtualService(VirtualService),
    DestinationRule(DestinationRule),
    EnvoyFilter(EnvoyFilter),
    Service(Service),
    Ingress(Ingress),
    Namespace(Namespace),
    Pod(Pod),
    Deployment(Deployment),
    CronJob(CronJob),
    DynamicObject(kube_core::DynamicObject),
}

impl From<Gateway> for Resource {
    fn from(t: Gateway) -> Self {
        Self::Gateway(t)
    }
}

impl From<VirtualService> for Resource {
    fn from(t: VirtualService) -> Self {
        Self::VirtualService(t)
    }
}
impl From<DestinationRule> for Resource {
    fn from(t: DestinationRule) -> Self {
        Self::DestinationRule(t)
    }
}
impl From<EnvoyFilter> for Resource {
    fn from(t: EnvoyFilter) -> Self {
        Self::EnvoyFilter(t)
    }
}

impl From<Service> for Resource {
    fn from(t: Service) -> Self {
        Self::Service(t)
    }
}

impl From<Ingress> for Resource {
    fn from(t: Ingress) -> Self {
        Self::Ingress(t)
    }
}

impl From<Namespace> for Resource {
    fn from(t: Namespace) -> Self {
        Self::Namespace(t)
    }
}

impl From<Pod> for Resource {
    fn from(t: Pod) -> Self {
        Self::Pod(t)
    }
}

impl From<Deployment> for Resource {
    fn from(t: Deployment) -> Self {
        Self::Deployment(t)
    }
}

impl From<CronJob> for Resource {
    fn from(t: CronJob) -> Self {
        Self::CronJob(t)
    }
}

impl From<DynamicObject> for Resource {
    fn from(t: DynamicObject) -> Self {
        Self::DynamicObject(t)
    }
}
