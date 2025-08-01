use crate::models::podman::pods::systemd_units_generate::{
    PodSystemdUnitsGenerate, PodSystemdUnitsGenerateOptions,
};

pub type ContainerSystemdUnitsGenerateOptions<'a> = PodSystemdUnitsGenerateOptions<'a>;

pub type ContainerSystemdUnitsGenerate = PodSystemdUnitsGenerate;
