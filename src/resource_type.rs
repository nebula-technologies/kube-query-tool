#[derive(Clone)]
pub enum ResourceType {
    Gateway,
    VirtualService,
    DestinationRule,
    EnvoyFilter,
    Service,
    Ingress,
    Namespace,
    Pod,
    Deployment,
    CronJob,
    DynamicObject,
}

impl From<String> for ResourceType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "gateway" => Self::Gateway,
            "virtualservice" => Self::VirtualService,
            "destinationrule" => Self::DestinationRule,
            "envoyfilter" => Self::EnvoyFilter,
            "service" => Self::Service,
            "ingress" => Self::Ingress,
            "namespace" => Self::Namespace,
            "pod" => Self::Pod,
            "deployment" => Self::Deployment,
            "cronjob" => Self::CronJob,
            _ => Self::DynamicObject,
        }
    }
}
