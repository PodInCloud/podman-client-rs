#[derive(Default)]
pub struct ContainerRestoreOptions<'a> {
    pub name: &'a str,
    pub file_locks: Option<bool>,
    pub ignore_root_fs: Option<bool>,
    pub ignore_static_ip: Option<bool>,
    pub ignore_static_mac: Option<bool>,
    pub ignore_volumes: Option<bool>,
    pub import: Option<bool>,
    pub keep: Option<bool>,
    pub name_import: Option<&'a str>,
    pub pod: Option<&'a str>,
    pub print_stats: Option<bool>,
    pub tcp_established: Option<bool>,
}
