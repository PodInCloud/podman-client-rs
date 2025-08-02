use crate::models::podman::common::health_check::HealthCheck;

pub struct ContainerHealthCheckOptions<'a> {
    pub name: &'a str,
}

pub type ContainerHealthCheck = HealthCheck;
