#[derive(Default)]
pub struct ContainerWaitOptions<'a> {
    pub name: &'a str,
    pub condition: Option<ContainerWaitConditionOptions>,
    pub interval: Option<&'a str>,
}

pub enum ContainerWaitConditionOptions {
    Configured,
    Created,
    Exited,
    Healthy,
    Initialized,
    Paused,
    Removing,
    Running,
    Stopped,
    Stopping,
    Unhealthy,
}

impl ContainerWaitConditionOptions {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Configured => "configured",
            Self::Created => "created",
            Self::Exited => "exited",
            Self::Healthy => "healthy",
            Self::Initialized => "initialized",
            Self::Paused => "paused",
            Self::Removing => "removing",
            Self::Running => "running",
            Self::Stopped => "stopped",
            Self::Stopping => "stopping",
            Self::Unhealthy => "unhealthy",
        }
    }
}

pub type ContainerWait = i32;
