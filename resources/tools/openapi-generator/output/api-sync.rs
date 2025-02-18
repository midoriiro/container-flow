use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`config_create`]
#[derive(Clone, Debug)]
pub struct ConfigCreateParams {
    pub body: Option<models::ConfigCreateRequest>,
}
/// struct for passing parameters to the method [`config_delete`]
#[derive(Clone, Debug)]
pub struct ConfigDeleteParams {
    /// ID of the config
    pub id: String,
}
/// struct for passing parameters to the method [`config_inspect`]
#[derive(Clone, Debug)]
pub struct ConfigInspectParams {
    /// ID of the config
    pub id: String,
}
/// struct for passing parameters to the method [`config_list`]
#[derive(Clone, Debug)]
pub struct ConfigListParams {
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the configs list.  Available filters:  - `id=<config id>` - `label=<key> or label=<key>=value` - `name=<config name>` - `names=<config name>`
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`config_update`]
#[derive(Clone, Debug)]
pub struct ConfigUpdateParams {
    /// The ID or name of the config
    pub id: String,
    /// The version number of the config object being updated. This is required to avoid conflicting writes.
    pub version: i64,
    /// The spec of the config to update. Currently, only the Labels field can be updated. All other fields must remain unchanged from the [ConfigInspect endpoint](#operation/ConfigInspect) response values.
    pub body: Option<models::ConfigSpec>,
}
/// struct for typed errors of method [`config_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigCreateError {
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`config_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigDeleteError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`config_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`config_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigListError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`config_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConfigUpdateError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn config_create(
    configuration: &configuration::Configuration,
    params: ConfigCreateParams,
) -> Result<models::IdResponse, Error<ConfigCreateError>> {
    let uri_str = format!("{}/configs/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ConfigCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn config_delete(
    configuration: &configuration::Configuration,
    params: ConfigDeleteParams,
) -> Result<(), Error<ConfigDeleteError>> {
    let uri_str = format!(
        "{}/configs/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ConfigDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn config_inspect(
    configuration: &configuration::Configuration,
    params: ConfigInspectParams,
) -> Result<models::Config, Error<ConfigInspectError>> {
    let uri_str = format!(
        "{}/configs/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ConfigInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn config_list(
    configuration: &configuration::Configuration,
    params: ConfigListParams,
) -> Result<Vec<models::Config>, Error<ConfigListError>> {
    let uri_str = format!("{}/configs", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ConfigListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn config_update(
    configuration: &configuration::Configuration,
    params: ConfigUpdateParams,
) -> Result<(), Error<ConfigUpdateError>> {
    let uri_str = format!(
        "{}/configs/{id}/update", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("version", &params.version.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ConfigUpdateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`container_archive`]
#[derive(Clone, Debug)]
pub struct ContainerArchiveParams {
    /// ID or name of the container
    pub id: String,
    /// Resource in the container’s filesystem to archive.
    pub path: String,
}
/// struct for passing parameters to the method [`container_archive_info`]
#[derive(Clone, Debug)]
pub struct ContainerArchiveInfoParams {
    /// ID or name of the container
    pub id: String,
    /// Resource in the container’s filesystem to archive.
    pub path: String,
}
/// struct for passing parameters to the method [`container_attach`]
#[derive(Clone, Debug)]
pub struct ContainerAttachParams {
    /// ID or name of the container
    pub id: String,
    /// Override the key sequence for detaching a container.Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    pub detach_keys: Option<String>,
    /// Replay previous logs from the container.  This is useful for attaching to a container that has started and you want to output everything since the container started.  If `stream` is also enabled, once all the previous output has been returned, it will seamlessly transition into streaming current output.
    pub logs: Option<bool>,
    /// Stream attached streams from the time the request was made onwards.
    pub stream: Option<bool>,
    /// Attach to `stdin`
    pub stdin: Option<bool>,
    /// Attach to `stdout`
    pub stdout: Option<bool>,
    /// Attach to `stderr`
    pub stderr: Option<bool>,
}
/// struct for passing parameters to the method [`container_attach_websocket`]
#[derive(Clone, Debug)]
pub struct ContainerAttachWebsocketParams {
    /// ID or name of the container
    pub id: String,
    /// Override the key sequence for detaching a container.Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,`, or `_`.
    pub detach_keys: Option<String>,
    /// Return logs
    pub logs: Option<bool>,
    /// Return stream
    pub stream: Option<bool>,
    /// Attach to `stdin`
    pub stdin: Option<bool>,
    /// Attach to `stdout`
    pub stdout: Option<bool>,
    /// Attach to `stderr`
    pub stderr: Option<bool>,
}
/// struct for passing parameters to the method [`container_changes`]
#[derive(Clone, Debug)]
pub struct ContainerChangesParams {
    /// ID or name of the container
    pub id: String,
}
/// struct for passing parameters to the method [`container_create`]
#[derive(Clone, Debug)]
pub struct ContainerCreateParams {
    /// Container to create
    pub body: models::ContainerCreateRequest,
    /// Assign the specified name to the container. Must match `/?[a-zA-Z0-9][a-zA-Z0-9_.-]+`.
    pub name: Option<String>,
    /// Platform in the format `os[/arch[/variant]]` used for image lookup.  When specified, the daemon checks if the requested image is present in the local image cache with the given OS and Architecture, and otherwise returns a `404` status.  If the option is not set, the host's native OS and Architecture are used to look up the image in the image cache. However, if no platform is passed and the given image does exist in the local image cache, but its OS or architecture does not match, the container is created with the available image, and a warning is added to the `Warnings` field in the response, for example;      WARNING: The requested image's platform (linux/arm64/v8) does not              match the detected host platform (linux/amd64) and no              specific platform was requested
    pub platform: Option<String>,
}
/// struct for passing parameters to the method [`container_delete`]
#[derive(Clone, Debug)]
pub struct ContainerDeleteParams {
    /// ID or name of the container
    pub id: String,
    /// Remove anonymous volumes associated with the container.
    pub v: Option<bool>,
    /// If the container is running, kill it before removing it.
    pub force: Option<bool>,
    /// Remove the specified link associated with the container.
    pub link: Option<bool>,
}
/// struct for passing parameters to the method [`container_export`]
#[derive(Clone, Debug)]
pub struct ContainerExportParams {
    /// ID or name of the container
    pub id: String,
}
/// struct for passing parameters to the method [`container_inspect`]
#[derive(Clone, Debug)]
pub struct ContainerInspectParams {
    /// ID or name of the container
    pub id: String,
    /// Return the size of container as fields `SizeRw` and `SizeRootFs`
    pub size: Option<bool>,
}
/// struct for passing parameters to the method [`container_kill`]
#[derive(Clone, Debug)]
pub struct ContainerKillParams {
    /// ID or name of the container
    pub id: String,
    /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
    pub signal: Option<String>,
}
/// struct for passing parameters to the method [`container_list`]
#[derive(Clone, Debug)]
pub struct ContainerListParams {
    /// Return all containers. By default, only running containers are shown.
    pub all: Option<bool>,
    /// Return this number of most recently created containers, including non-running ones.
    pub limit: Option<i32>,
    /// Return the size of container as fields `SizeRw` and `SizeRootFs`.
    pub size: Option<bool>,
    /// Filters to process on the container list, encoded as JSON (a `map[string][]string`). For example, `{\"status\": [\"paused\"]}` will only return paused containers.  Available filters:  - `ancestor`=(`<image-name>[:<tag>]`, `<image id>`, or `<image@digest>`) - `before`=(`<container id>` or `<container name>`) - `expose`=(`<port>[/<proto>]`|`<startport-endport>/[<proto>]`) - `exited=<int>` containers with exit code of `<int>` - `health`=(`starting`|`healthy`|`unhealthy`|`none`) - `id=<ID>` a container's ID - `isolation=`(`default`|`process`|`hyperv`) (Windows daemon only) - `is-task=`(`true`|`false`) - `label=key` or `label=\"key=value\"` of a container label - `name=<name>` a container's name - `network`=(`<network id>` or `<network name>`) - `publish`=(`<port>[/<proto>]`|`<startport-endport>/[<proto>]`) - `since`=(`<container id>` or `<container name>`) - `status=`(`created`|`restarting`|`running`|`removing`|`paused`|`exited`|`dead`) - `volume`=(`<volume name>` or `<mount point destination>`)
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`container_logs`]
#[derive(Clone, Debug)]
pub struct ContainerLogsParams {
    /// ID or name of the container
    pub id: String,
    /// Keep connection after returning logs.
    pub follow: Option<bool>,
    /// Return logs from `stdout`
    pub stdout: Option<bool>,
    /// Return logs from `stderr`
    pub stderr: Option<bool>,
    /// Only return logs since this time, as a UNIX timestamp
    pub since: Option<i32>,
    /// Only return logs before this time, as a UNIX timestamp
    pub until: Option<i32>,
    /// Add timestamps to every log line
    pub timestamps: Option<bool>,
    /// Only return this number of log lines from the end of the logs. Specify as an integer or `all` to output all log lines.
    pub tail: Option<String>,
}
/// struct for passing parameters to the method [`container_pause`]
#[derive(Clone, Debug)]
pub struct ContainerPauseParams {
    /// ID or name of the container
    pub id: String,
}
/// struct for passing parameters to the method [`container_prune`]
#[derive(Clone, Debug)]
pub struct ContainerPruneParams {
    /// Filters to process on the prune list, encoded as JSON (a `map[string][]string`).  Available filters: - `until=<timestamp>` Prune containers created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time. - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune containers with (or without, in case `label!=...` is used) the specified labels.
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`container_rename`]
#[derive(Clone, Debug)]
pub struct ContainerRenameParams {
    /// ID or name of the container
    pub id: String,
    /// New name for the container
    pub name: String,
}
/// struct for passing parameters to the method [`container_resize`]
#[derive(Clone, Debug)]
pub struct ContainerResizeParams {
    /// ID or name of the container
    pub id: String,
    /// Height of the TTY session in characters
    pub h: i32,
    /// Width of the TTY session in characters
    pub w: i32,
}
/// struct for passing parameters to the method [`container_restart`]
#[derive(Clone, Debug)]
pub struct ContainerRestartParams {
    /// ID or name of the container
    pub id: String,
    /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
    pub signal: Option<String>,
    /// Number of seconds to wait before killing the container
    pub t: Option<i32>,
}
/// struct for passing parameters to the method [`container_start`]
#[derive(Clone, Debug)]
pub struct ContainerStartParams {
    /// ID or name of the container
    pub id: String,
    /// Override the key sequence for detaching a container. Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    pub detach_keys: Option<String>,
}
/// struct for passing parameters to the method [`container_stats`]
#[derive(Clone, Debug)]
pub struct ContainerStatsParams {
    /// ID or name of the container
    pub id: String,
    /// Stream the output. If false, the stats will be output once and then it will disconnect.
    pub stream: Option<bool>,
    /// Only get a single stat instead of waiting for 2 cycles. Must be used with `stream=false`.
    pub one_shot: Option<bool>,
}
/// struct for passing parameters to the method [`container_stop`]
#[derive(Clone, Debug)]
pub struct ContainerStopParams {
    /// ID or name of the container
    pub id: String,
    /// Signal to send to the container as an integer or string (e.g. `SIGINT`).
    pub signal: Option<String>,
    /// Number of seconds to wait before killing the container
    pub t: Option<i32>,
}
/// struct for passing parameters to the method [`container_top`]
#[derive(Clone, Debug)]
pub struct ContainerTopParams {
    /// ID or name of the container
    pub id: String,
    /// The arguments to pass to `ps`. For example, `aux`
    pub ps_args: Option<String>,
}
/// struct for passing parameters to the method [`container_unpause`]
#[derive(Clone, Debug)]
pub struct ContainerUnpauseParams {
    /// ID or name of the container
    pub id: String,
}
/// struct for passing parameters to the method [`container_update`]
#[derive(Clone, Debug)]
pub struct ContainerUpdateParams {
    /// ID or name of the container
    pub id: String,
    pub update: models::ContainerUpdateRequest,
}
/// struct for passing parameters to the method [`container_wait`]
#[derive(Clone, Debug)]
pub struct ContainerWaitParams {
    /// ID or name of the container
    pub id: String,
    /// Wait until a container state reaches the given condition.  Defaults to `not-running` if omitted or empty.
    pub condition: Option<String>,
}
/// struct for passing parameters to the method [`put_container_archive`]
#[derive(Clone, Debug)]
pub struct PutContainerArchiveParams {
    /// ID or name of the container
    pub id: String,
    /// Path to a directory in the container to extract the archive’s contents into.
    pub path: String,
    /// The input stream must be a tar archive compressed with one of the following algorithms: `identity` (no compression), `gzip`, `bzip2`, or `xz`.
    pub input_stream: std::path::PathBuf,
    /// If `1`, `true`, or `True` then it will be an error if unpacking the given content would cause an existing directory to be replaced with a non-directory and vice versa.
    pub no_overwrite_dir_non_dir: Option<String>,
    /// If `1`, `true`, then it will copy UID/GID maps to the dest file or dir
    pub copy_uidgid: Option<String>,
}
/// struct for typed errors of method [`container_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerArchiveError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_archive_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerArchiveInfoError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_attach`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerAttachError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_attach_websocket`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerAttachWebsocketError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_changes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerChangesError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerCreateError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerDeleteError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_export`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerExportError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_kill`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerKillError {
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerListError {
    Status400(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerLogsError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_pause`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerPauseError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_prune`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerPruneError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_rename`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerRenameError {
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_resize`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerResizeError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_restart`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerRestartError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_start`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerStartError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_stats`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerStatsError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_stop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerStopError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_top`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerTopError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_unpause`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerUnpauseError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerUpdateError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`container_wait`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerWaitError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`put_container_archive`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PutContainerArchiveError {
    Status400(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// Get a tar archive of a resource in the filesystem of container id.
pub fn container_archive(
    configuration: &configuration::Configuration,
    params: ContainerArchiveParams,
) -> Result<(), Error<ContainerArchiveError>> {
    let uri_str = format!(
        "{}/containers/{id}/archive", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    req_builder = req_builder.query(&[("path", &params.path.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerArchiveError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// A response header `X-Docker-Container-Path-Stat` is returned, containing a base64 - encoded JSON object with some filesystem header information about the path.
pub fn container_archive_info(
    configuration: &configuration::Configuration,
    params: ContainerArchiveInfoParams,
) -> Result<(), Error<ContainerArchiveInfoError>> {
    let uri_str = format!(
        "{}/containers/{id}/archive", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::HEAD, &uri_str);
    req_builder = req_builder.query(&[("path", &params.path.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerArchiveInfoError> = serde_json::from_str(&content)
            .ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Attach to a container to read its output or send it input. You can attach to the same container multiple times and you can reattach to containers that have been detached.  Either the `stream` or `logs` parameter must be `true` for this endpoint to do anything.  See the [documentation for the `docker attach` command](https://docs.docker.com/engine/reference/commandline/attach/) for more details.  ### Hijacking  This endpoint hijacks the HTTP connection to transport `stdin`, `stdout`, and `stderr` on the same socket.  This is the response from the daemon for an attach request:  ``` HTTP/1.1 200 OK Content-Type: application/vnd.docker.raw-stream  [STREAM] ```  After the headers and two new lines, the TCP connection can now be used for raw, bidirectional communication between the client and server.  To hint potential proxies about connection hijacking, the Docker client can also optionally send connection upgrade headers.  For example, the client sends this request to upgrade the connection:  ``` POST /containers/16253994b7c4/attach?stream=1&stdout=1 HTTP/1.1 Upgrade: tcp Connection: Upgrade ```  The Docker daemon will respond with a `101 UPGRADED` response, and will similarly follow with the raw stream:  ``` HTTP/1.1 101 UPGRADED Content-Type: application/vnd.docker.raw-stream Connection: Upgrade Upgrade: tcp  [STREAM] ```  ### Stream format  When the TTY setting is disabled in [`POST /containers/create`](#operation/ContainerCreate), the HTTP Content-Type header is set to application/vnd.docker.multiplexed-stream and the stream over the hijacked connected is multiplexed to separate out `stdout` and `stderr`. The stream consists of a series of frames, each containing a header and a payload.  The header contains the information which the stream writes (`stdout` or `stderr`). It also contains the size of the associated frame encoded in the last four bytes (`uint32`).  It is encoded on the first eight bytes like this:  ```go header := [8]byte{STREAM_TYPE, 0, 0, 0, SIZE1, SIZE2, SIZE3, SIZE4} ```  `STREAM_TYPE` can be:  - 0: `stdin` (is written on `stdout`) - 1: `stdout` - 2: `stderr`  `SIZE1, SIZE2, SIZE3, SIZE4` are the four bytes of the `uint32` size encoded as big endian.  Following the header is the payload, which is the specified number of bytes of `STREAM_TYPE`.  The simplest way to implement this protocol is the following:  1. Read 8 bytes. 2. Choose `stdout` or `stderr` depending on the first byte. 3. Extract the frame size from the last four bytes. 4. Read the extracted size and output it on the correct output. 5. Goto 1.  ### Stream format when using a TTY  When the TTY setting is enabled in [`POST /containers/create`](#operation/ContainerCreate), the stream is not multiplexed. The data exchanged over the hijacked connection is simply the raw data from the process PTY and client's `stdin`.
pub fn container_attach(
    configuration: &configuration::Configuration,
    params: ContainerAttachParams,
) -> Result<(), Error<ContainerAttachError>> {
    let uri_str = format!(
        "{}/containers/{id}/attach", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.detach_keys {
        req_builder = req_builder.query(&[("detachKeys", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.logs {
        req_builder = req_builder.query(&[("logs", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stream {
        req_builder = req_builder.query(&[("stream", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stdin {
        req_builder = req_builder.query(&[("stdin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stdout {
        req_builder = req_builder.query(&[("stdout", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stderr {
        req_builder = req_builder.query(&[("stderr", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerAttachError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_attach_websocket(
    configuration: &configuration::Configuration,
    params: ContainerAttachWebsocketParams,
) -> Result<(), Error<ContainerAttachWebsocketError>> {
    let uri_str = format!(
        "{}/containers/{id}/attach/ws", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.detach_keys {
        req_builder = req_builder.query(&[("detachKeys", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.logs {
        req_builder = req_builder.query(&[("logs", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stream {
        req_builder = req_builder.query(&[("stream", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stdin {
        req_builder = req_builder.query(&[("stdin", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stdout {
        req_builder = req_builder.query(&[("stdout", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stderr {
        req_builder = req_builder.query(&[("stderr", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerAttachWebsocketError> = serde_json::from_str(
                &content,
            )
            .ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Returns which files in a container's filesystem have been added, deleted, or modified. The `Kind` of modification can be one of:  - `0`: Modified (\"C\") - `1`: Added (\"A\") - `2`: Deleted (\"D\")
pub fn container_changes(
    configuration: &configuration::Configuration,
    params: ContainerChangesParams,
) -> Result<Vec<models::FilesystemChange>, Error<ContainerChangesError>> {
    let uri_str = format!(
        "{}/containers/{id}/changes", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerChangesError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_create(
    configuration: &configuration::Configuration,
    params: ContainerCreateParams,
) -> Result<models::ContainerCreateResponse, Error<ContainerCreateError>> {
    let uri_str = format!("{}/containers/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_delete(
    configuration: &configuration::Configuration,
    params: ContainerDeleteParams,
) -> Result<(), Error<ContainerDeleteError>> {
    let uri_str = format!(
        "{}/containers/{id}", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref param_value) = params.v {
        req_builder = req_builder.query(&[("v", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.link {
        req_builder = req_builder.query(&[("link", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Export the contents of a container as a tarball.
pub fn container_export(
    configuration: &configuration::Configuration,
    params: ContainerExportParams,
) -> Result<(), Error<ContainerExportError>> {
    let uri_str = format!(
        "{}/containers/{id}/export", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerExportError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Return low-level information about a container.
pub fn container_inspect(
    configuration: &configuration::Configuration,
    params: ContainerInspectParams,
) -> Result<models::ContainerInspectResponse, Error<ContainerInspectError>> {
    let uri_str = format!(
        "{}/containers/{id}/json", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.size {
        req_builder = req_builder.query(&[("size", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Send a POSIX signal to a container, defaulting to killing to the container.
pub fn container_kill(
    configuration: &configuration::Configuration,
    params: ContainerKillParams,
) -> Result<(), Error<ContainerKillError>> {
    let uri_str = format!(
        "{}/containers/{id}/kill", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.signal {
        req_builder = req_builder.query(&[("signal", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerKillError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Returns a list of containers. For details on the format, see the [inspect endpoint](#operation/ContainerInspect).  Note that it uses a different, smaller representation of a container than inspecting a single container. For example, the list of linked containers is not propagated .
pub fn container_list(
    configuration: &configuration::Configuration,
    params: ContainerListParams,
) -> Result<Vec<models::ContainerSummary>, Error<ContainerListError>> {
    let uri_str = format!("{}/containers/json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.all {
        req_builder = req_builder.query(&[("all", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.size {
        req_builder = req_builder.query(&[("size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Get `stdout` and `stderr` logs from a container.  Note: This endpoint works only for containers with the `json-file` or `journald` logging driver.
pub fn container_logs(
    configuration: &configuration::Configuration,
    params: ContainerLogsParams,
) -> Result<reqwest::blocking::Response, Error<ContainerLogsError>> {
    let uri_str = format!(
        "{}/containers/{id}/logs", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stdout {
        req_builder = req_builder.query(&[("stdout", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stderr {
        req_builder = req_builder.query(&[("stderr", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.until {
        req_builder = req_builder.query(&[("until", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.timestamps {
        req_builder = req_builder.query(&[("timestamps", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.tail {
        req_builder = req_builder.query(&[("tail", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerLogsError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Use the freezer cgroup to suspend all processes in a container.  Traditionally, when suspending a process the `SIGSTOP` signal is used, which is observable by the process being suspended. With the freezer cgroup the process is unaware, and unable to capture, that it is being suspended, and subsequently resumed.
pub fn container_pause(
    configuration: &configuration::Configuration,
    params: ContainerPauseParams,
) -> Result<(), Error<ContainerPauseError>> {
    let uri_str = format!(
        "{}/containers/{id}/pause", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerPauseError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_prune(
    configuration: &configuration::Configuration,
    params: ContainerPruneParams,
) -> Result<models::ContainerPruneResponse, Error<ContainerPruneError>> {
    let uri_str = format!("{}/containers/prune", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerPruneError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_rename(
    configuration: &configuration::Configuration,
    params: ContainerRenameParams,
) -> Result<(), Error<ContainerRenameError>> {
    let uri_str = format!(
        "{}/containers/{id}/rename", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("name", &params.name.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerRenameError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Resize the TTY for a container.
pub fn container_resize(
    configuration: &configuration::Configuration,
    params: ContainerResizeParams,
) -> Result<(), Error<ContainerResizeError>> {
    let uri_str = format!(
        "{}/containers/{id}/resize", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("h", &params.h.to_string())]);
    req_builder = req_builder.query(&[("w", &params.w.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerResizeError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_restart(
    configuration: &configuration::Configuration,
    params: ContainerRestartParams,
) -> Result<(), Error<ContainerRestartError>> {
    let uri_str = format!(
        "{}/containers/{id}/restart", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.signal {
        req_builder = req_builder.query(&[("signal", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.t {
        req_builder = req_builder.query(&[("t", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerRestartError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_start(
    configuration: &configuration::Configuration,
    params: ContainerStartParams,
) -> Result<(), Error<ContainerStartError>> {
    let uri_str = format!(
        "{}/containers/{id}/start", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.detach_keys {
        req_builder = req_builder.query(&[("detachKeys", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerStartError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// This endpoint returns a live stream of a container’s resource usage statistics.  The `precpu_stats` is the CPU statistic of the *previous* read, and is used to calculate the CPU usage percentage. It is not an exact copy of the `cpu_stats` field.  If either `precpu_stats.online_cpus` or `cpu_stats.online_cpus` is nil then for compatibility with older daemons the length of the corresponding `cpu_usage.percpu_usage` array should be used.  On a cgroup v2 host, the following fields are not set * `blkio_stats`: all fields other than `io_service_bytes_recursive` * `cpu_stats`: `cpu_usage.percpu_usage` * `memory_stats`: `max_usage` and `failcnt` Also, `memory_stats.stats` fields are incompatible with cgroup v1.  To calculate the values shown by the `stats` command of the docker cli tool the following formulas can be used: * used_memory = `memory_stats.usage - memory_stats.stats.cache` * available_memory = `memory_stats.limit` * Memory usage % = `(used_memory / available_memory) * 100.0` * cpu_delta = `cpu_stats.cpu_usage.total_usage - precpu_stats.cpu_usage.total_usage` * system_cpu_delta = `cpu_stats.system_cpu_usage - precpu_stats.system_cpu_usage` * number_cpus = `length(cpu_stats.cpu_usage.percpu_usage)` or `cpu_stats.online_cpus` * CPU usage % = `(cpu_delta / system_cpu_delta) * number_cpus * 100.0`
pub fn container_stats(
    configuration: &configuration::Configuration,
    params: ContainerStatsParams,
) -> Result<serde_json::Value, Error<ContainerStatsError>> {
    let uri_str = format!(
        "{}/containers/{id}/stats", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.stream {
        req_builder = req_builder.query(&[("stream", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.one_shot {
        req_builder = req_builder.query(&[("one-shot", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerStatsError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn container_stop(
    configuration: &configuration::Configuration,
    params: ContainerStopParams,
) -> Result<(), Error<ContainerStopError>> {
    let uri_str = format!(
        "{}/containers/{id}/stop", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.signal {
        req_builder = req_builder.query(&[("signal", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.t {
        req_builder = req_builder.query(&[("t", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerStopError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// On Unix systems, this is done by running the `ps` command. This endpoint is not supported on Windows.
pub fn container_top(
    configuration: &configuration::Configuration,
    params: ContainerTopParams,
) -> Result<models::ContainerTopResponse, Error<ContainerTopError>> {
    let uri_str = format!(
        "{}/containers/{id}/top", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.ps_args {
        req_builder = req_builder.query(&[("ps_args", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerTopError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Resume a container which has been paused.
pub fn container_unpause(
    configuration: &configuration::Configuration,
    params: ContainerUnpauseParams,
) -> Result<(), Error<ContainerUnpauseError>> {
    let uri_str = format!(
        "{}/containers/{id}/unpause", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerUnpauseError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Change various configuration options of a container without having to recreate it.
pub fn container_update(
    configuration: &configuration::Configuration,
    params: ContainerUpdateParams,
) -> Result<models::ContainerUpdateResponse, Error<ContainerUpdateError>> {
    let uri_str = format!(
        "{}/containers/{id}/update", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.update);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerUpdateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Block until a container stops, then returns the exit code.
pub fn container_wait(
    configuration: &configuration::Configuration,
    params: ContainerWaitParams,
) -> Result<models::ContainerWaitResponse, Error<ContainerWaitError>> {
    let uri_str = format!(
        "{}/containers/{id}/wait", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.condition {
        req_builder = req_builder.query(&[("condition", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerWaitError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Upload a tar archive to be extracted to a path in the filesystem of container id. `path` parameter is asserted to be a directory. If it exists as a file, 400 error will be returned with message \"not a directory\".
pub fn put_container_archive(
    configuration: &configuration::Configuration,
    params: PutContainerArchiveParams,
) -> Result<(), Error<PutContainerArchiveError>> {
    let uri_str = format!(
        "{}/containers/{id}/archive", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);
    req_builder = req_builder.query(&[("path", &params.path.to_string())]);
    if let Some(ref param_value) = params.no_overwrite_dir_non_dir {
        req_builder = req_builder
            .query(&[("noOverwriteDirNonDir", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.copy_uidgid {
        req_builder = req_builder.query(&[("copyUIDGID", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.body(params.input_stream);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PutContainerArchiveError> = serde_json::from_str(&content)
            .ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`distribution_inspect`]
#[derive(Clone, Debug)]
pub struct DistributionInspectParams {
    /// Image name or id
    pub name: String,
}
/// struct for typed errors of method [`distribution_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DistributionInspectError {
    Status401(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// Return image digest and platform information by contacting the registry.
pub fn distribution_inspect(
    configuration: &configuration::Configuration,
    params: DistributionInspectParams,
) -> Result<models::DistributionInspect, Error<DistributionInspectError>> {
    let uri_str = format!(
        "{}/distribution/{name}/json", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<DistributionInspectError> = serde_json::from_str(&content)
            .ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`container_exec`]
#[derive(Clone, Debug)]
pub struct ContainerExecParams {
    /// ID or name of container
    pub id: String,
    /// Exec configuration
    pub exec_config: models::ExecConfig,
}
/// struct for passing parameters to the method [`exec_inspect`]
#[derive(Clone, Debug)]
pub struct ExecInspectParams {
    /// Exec instance ID
    pub id: String,
}
/// struct for passing parameters to the method [`exec_resize`]
#[derive(Clone, Debug)]
pub struct ExecResizeParams {
    /// Exec instance ID
    pub id: String,
    /// Height of the TTY session in characters
    pub h: i32,
    /// Width of the TTY session in characters
    pub w: i32,
}
/// struct for passing parameters to the method [`exec_start`]
#[derive(Clone, Debug)]
pub struct ExecStartParams {
    /// Exec instance ID
    pub id: String,
    pub exec_start_config: Option<models::ExecStartConfig>,
}
/// struct for typed errors of method [`container_exec`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContainerExecError {
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`exec_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`exec_resize`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecResizeError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`exec_start`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExecStartError {
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// Run a command inside a running container.
pub fn container_exec(
    configuration: &configuration::Configuration,
    params: ContainerExecParams,
) -> Result<models::IdResponse, Error<ContainerExecError>> {
    let uri_str = format!(
        "{}/containers/{id}/exec", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.exec_config);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ContainerExecError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Return low-level information about an exec instance.
pub fn exec_inspect(
    configuration: &configuration::Configuration,
    params: ExecInspectParams,
) -> Result<models::ExecInspectResponse, Error<ExecInspectError>> {
    let uri_str = format!(
        "{}/exec/{id}/json", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ExecInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Resize the TTY session used by an exec instance. This endpoint only works if `tty` was specified as part of creating and starting the exec instance.
pub fn exec_resize(
    configuration: &configuration::Configuration,
    params: ExecResizeParams,
) -> Result<(), Error<ExecResizeError>> {
    let uri_str = format!(
        "{}/exec/{id}/resize", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("h", &params.h.to_string())]);
    req_builder = req_builder.query(&[("w", &params.w.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ExecResizeError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Starts a previously set up exec instance. If detach is true, this endpoint returns immediately after starting the command. Otherwise, it sets up an interactive session with the command.
pub fn exec_start(
    configuration: &configuration::Configuration,
    params: ExecStartParams,
) -> Result<(), Error<ExecStartError>> {
    let uri_str = format!(
        "{}/exec/{id}/start", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.exec_start_config);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ExecStartError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`build_prune`]
#[derive(Clone, Debug)]
pub struct BuildPruneParams {
    /// Amount of disk space in bytes to keep for cache
    pub keep_storage: Option<i64>,
    /// Remove all types of build cache
    pub all: Option<bool>,
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the list of build cache objects.  Available filters:  - `until=<timestamp>` remove cache older than `<timestamp>`. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon's local time. - `id=<id>` - `parent=<id>` - `type=<string>` - `description=<string>` - `inuse` - `shared` - `private`
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`image_build`]
#[derive(Clone, Debug)]
pub struct ImageBuildParams {
    /// Path within the build context to the `Dockerfile`. This is ignored if `remote` is specified and points to an external `Dockerfile`.
    pub dockerfile: Option<String>,
    /// A name and optional tag to apply to the image in the `name:tag` format. If you omit the tag the default `latest` value is assumed. You can provide several `t` parameters.
    pub t: Option<String>,
    /// Extra hosts to add to /etc/hosts
    pub extrahosts: Option<String>,
    /// A Git repository URI or HTTP/HTTPS context URI. If the URI points to a single text file, the file’s contents are placed into a file called `Dockerfile` and the image is built from that file. If the URI points to a tarball, the file is downloaded by the daemon and the contents therein used as the context for the build. If the URI points to a tarball and the `dockerfile` parameter is also specified, there must be a file with the corresponding path inside the tarball.
    pub remote: Option<String>,
    /// Suppress verbose build output.
    pub q: Option<bool>,
    /// Do not use the cache when building the image.
    pub nocache: Option<bool>,
    /// JSON array of images used for build cache resolution.
    pub cachefrom: Option<String>,
    /// Attempt to pull the image even if an older image exists locally.
    pub pull: Option<String>,
    /// Remove intermediate containers after a successful build.
    pub rm: Option<bool>,
    /// Always remove intermediate containers, even upon failure.
    pub forcerm: Option<bool>,
    /// Set memory limit for build.
    pub memory: Option<i32>,
    /// Total memory (memory + swap). Set as `-1` to disable swap.
    pub memswap: Option<i32>,
    /// CPU shares (relative weight).
    pub cpushares: Option<i32>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    pub cpusetcpus: Option<String>,
    /// The length of a CPU period in microseconds.
    pub cpuperiod: Option<i32>,
    /// Microseconds of CPU time that the container can get in a CPU period.
    pub cpuquota: Option<i32>,
    /// JSON map of string pairs for build-time variables. Users pass these values at build-time. Docker uses the buildargs as the environment context for commands run via the `Dockerfile` RUN instruction, or for variable expansion in other `Dockerfile` instructions. This is not meant for passing secret values.  For example, the build arg `FOO=bar` would become `{\"FOO\":\"bar\"}` in JSON. This would result in the query parameter `buildargs={\"FOO\":\"bar\"}`. Note that `{\"FOO\":\"bar\"}` should be URI component encoded.  [Read more about the buildargs instruction.](https://docs.docker.com/engine/reference/builder/#arg)
    pub buildargs: Option<String>,
    /// Size of `/dev/shm` in bytes. The size must be greater than 0. If omitted the system uses 64MB.
    pub shmsize: Option<i32>,
    /// Squash the resulting images layers into a single layer. *(Experimental release only.)*
    pub squash: Option<bool>,
    /// Arbitrary key/value labels to set on the image, as a JSON map of string pairs.
    pub labels: Option<String>,
    /// Sets the networking mode for the run commands during build. Supported standard values are: `bridge`, `host`, `none`, and `container:<name|id>`. Any other value is taken as a custom network's name or ID to which this container should connect to.
    pub networkmode: Option<String>,
    pub content_type: Option<String>,
    /// This is a base64-encoded JSON object with auth configurations for multiple registries that a build may refer to.  The key is a registry URL, and the value is an auth configuration object, [as described in the authentication section](#section/Authentication). For example:  ``` {   \"docker.example.com\": {     \"username\": \"janedoe\",     \"password\": \"hunter2\"   },   \"https://index.docker.io/v1/\": {     \"username\": \"mobydock\",     \"password\": \"conta1n3rize14\"   } } ```  Only the registry domain name (and port if not the default 443) are required. However, for legacy reasons, the Docker Hub registry must be specified with both a `https://` prefix and a `/v1/` suffix even though Docker will prefer to use the v2 registry API.
    pub x_registry_config: Option<String>,
    /// Platform in the format os[/arch[/variant]]
    pub platform: Option<String>,
    /// Target build stage
    pub target: Option<String>,
    /// BuildKit output configuration
    pub outputs: Option<String>,
    /// Version of the builder backend to use.  - `1` is the first generation classic (deprecated) builder in the Docker daemon (default) - `2` is [BuildKit](https://github.com/moby/buildkit)
    pub version: Option<String>,
    /// A tar archive compressed with one of the following algorithms: identity (no compression), gzip, bzip2, xz.
    pub input_stream: Option<std::path::PathBuf>,
}
/// struct for passing parameters to the method [`image_commit`]
#[derive(Clone, Debug)]
pub struct ImageCommitParams {
    /// The ID or name of the container to commit
    pub container: Option<String>,
    /// Repository name for the created image
    pub repo: Option<String>,
    /// Tag name for the create image
    pub tag: Option<String>,
    /// Commit message
    pub comment: Option<String>,
    /// Author of the image (e.g., `John Hannibal Smith <hannibal@a-team.com>`)
    pub author: Option<String>,
    /// Whether to pause the container before committing
    pub pause: Option<bool>,
    /// `Dockerfile` instructions to apply while committing
    pub changes: Option<String>,
    /// The container configuration
    pub container_config: Option<models::ContainerConfig>,
}
/// struct for passing parameters to the method [`image_create`]
#[derive(Clone, Debug)]
pub struct ImageCreateParams {
    /// Name of the image to pull. The name may include a tag or digest. This parameter may only be used when pulling an image. The pull is cancelled if the HTTP connection is closed.
    pub from_image: Option<String>,
    /// Source to import. The value may be a URL from which the image can be retrieved or `-` to read the image from the request body. This parameter may only be used when importing an image.
    pub from_src: Option<String>,
    /// Repository name given to an image when it is imported. The repo may include a tag. This parameter may only be used when importing an image.
    pub repo: Option<String>,
    /// Tag or digest. If empty when pulling an image, this causes all tags for the given image to be pulled.
    pub tag: Option<String>,
    /// Set commit message for imported image.
    pub message: Option<String>,
    /// A base64url-encoded auth configuration.  Refer to the [authentication section](#section/Authentication) for details.
    pub x_registry_auth: Option<String>,
    /// Apply `Dockerfile` instructions to the image that is created, for example: `changes=ENV DEBUG=true`. Note that `ENV DEBUG=true` should be URI component encoded.  Supported `Dockerfile` instructions: `CMD`|`ENTRYPOINT`|`ENV`|`EXPOSE`|`ONBUILD`|`USER`|`VOLUME`|`WORKDIR`
    pub changes: Option<Vec<String>>,
    /// Platform in the format os[/arch[/variant]].  When used in combination with the `fromImage` option, the daemon checks if the given image is present in the local image cache with the given OS and Architecture, and otherwise attempts to pull the image. If the option is not set, the host's native OS and Architecture are used. If the given image does not exist in the local image cache, the daemon attempts to pull the image with the host's native OS and Architecture. If the given image does exists in the local image cache, but its OS or architecture does not match, a warning is produced.  When used with the `fromSrc` option to import an image from an archive, this option sets the platform information for the imported image. If the option is not set, the host's native OS and Architecture are used for the imported image.
    pub platform: Option<String>,
    /// Image content if the value `-` has been specified in fromSrc query parameter
    pub input_image: Option<String>,
}
/// struct for passing parameters to the method [`image_delete`]
#[derive(Clone, Debug)]
pub struct ImageDeleteParams {
    /// Image name or ID
    pub name: String,
    /// Remove the image even if it is being used by stopped containers or has other tags
    pub force: Option<bool>,
    /// Do not delete untagged parent images
    pub noprune: Option<bool>,
}
/// struct for passing parameters to the method [`image_get`]
#[derive(Clone, Debug)]
pub struct ImageGetParams {
    /// Image name or ID
    pub name: String,
}
/// struct for passing parameters to the method [`image_get_all`]
#[derive(Clone, Debug)]
pub struct ImageGetAllParams {
    /// Image names to filter by
    pub names: Option<Vec<String>>,
}
/// struct for passing parameters to the method [`image_history`]
#[derive(Clone, Debug)]
pub struct ImageHistoryParams {
    /// Image name or ID
    pub name: String,
}
/// struct for passing parameters to the method [`image_inspect`]
#[derive(Clone, Debug)]
pub struct ImageInspectParams {
    /// Image name or id
    pub name: String,
}
/// struct for passing parameters to the method [`image_list`]
#[derive(Clone, Debug)]
pub struct ImageListParams {
    /// Show all images. Only images from a final layer (no children) are shown by default.
    pub all: Option<bool>,
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the images list.  Available filters:  - `before`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`) - `dangling=true` - `label=key` or `label=\"key=value\"` of an image label - `reference`=(`<image-name>[:<tag>]`) - `since`=(`<image-name>[:<tag>]`,  `<image id>` or `<image@digest>`) - `until=<timestamp>`
    pub filters: Option<String>,
    /// Compute and show shared size as a `SharedSize` field on each image.
    pub shared_size: Option<bool>,
    /// Show digest information as a `RepoDigests` field on each image.
    pub digests: Option<bool>,
    /// Include `Manifests` in the image summary.
    pub manifests: Option<bool>,
}
/// struct for passing parameters to the method [`image_load`]
#[derive(Clone, Debug)]
pub struct ImageLoadParams {
    /// Suppress progress details during load.
    pub quiet: Option<bool>,
    /// Tar archive containing images
    pub images_tarball: Option<std::path::PathBuf>,
}
/// struct for passing parameters to the method [`image_prune`]
#[derive(Clone, Debug)]
pub struct ImagePruneParams {
    /// Filters to process on the prune list, encoded as JSON (a `map[string][]string`). Available filters:  - `dangling=<boolean>` When set to `true` (or `1`), prune only    unused *and* untagged images. When set to `false`    (or `0`), all unused images are pruned. - `until=<string>` Prune images created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time. - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune images with (or without, in case `label!=...` is used) the specified labels.
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`image_push`]
#[derive(Clone, Debug)]
pub struct ImagePushParams {
    /// Name of the image to push. For example, `registry.example.com/myimage`. The image must be present in the local image store with the same name.  The name should be provided without tag; if a tag is provided, it is ignored. For example, `registry.example.com/myimage:latest` is considered equivalent to `registry.example.com/myimage`.  Use the `tag` parameter to specify the tag to push.
    pub name: String,
    /// A base64url-encoded auth configuration.  Refer to the [authentication section](#section/Authentication) for details.
    pub x_registry_auth: String,
    /// Tag of the image to push. For example, `latest`. If no tag is provided, all tags of the given image that are present in the local image store are pushed.
    pub tag: Option<String>,
    /// JSON-encoded OCI platform to select the platform-variant to push. If not provided, all available variants will attempt to be pushed.  If the daemon provides a multi-platform image store, this selects the platform-variant to push to the registry. If the image is a single-platform image, or if the multi-platform image does not provide a variant matching the given platform, an error is returned.  Example: `{\"os\": \"linux\", \"architecture\": \"arm\", \"variant\": \"v5\"}`
    pub platform: Option<String>,
}
/// struct for passing parameters to the method [`image_search`]
#[derive(Clone, Debug)]
pub struct ImageSearchParams {
    /// Term to search
    pub term: String,
    /// Maximum number of results to return
    pub limit: Option<i32>,
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the images list. Available filters:  - `is-official=(true|false)` - `stars=<number>` Matches images that has at least 'number' stars.
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`image_tag`]
#[derive(Clone, Debug)]
pub struct ImageTagParams {
    /// Image name or ID to tag.
    pub name: String,
    /// The repository to tag in. For example, `someuser/someimage`.
    pub repo: Option<String>,
    /// The name of the new tag.
    pub tag: Option<String>,
}
/// struct for typed errors of method [`build_prune`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildPruneError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_build`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageBuildError {
    Status400(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_commit`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageCommitError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageCreateError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageDeleteError {
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageGetError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_get_all`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageGetAllError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_history`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageHistoryError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageListError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_load`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageLoadError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_prune`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImagePruneError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_push`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImagePushError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_search`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageSearchError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`image_tag`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageTagError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn build_prune(
    configuration: &configuration::Configuration,
    params: BuildPruneParams,
) -> Result<models::BuildPruneResponse, Error<BuildPruneError>> {
    let uri_str = format!("{}/build/prune", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.keep_storage {
        req_builder = req_builder.query(&[("keep-storage", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.all {
        req_builder = req_builder.query(&[("all", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<BuildPruneError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Build an image from a tar archive with a `Dockerfile` in it.  The `Dockerfile` specifies how the image is built from the tar archive. It is typically in the archive's root, but can be at a different path or have a different name by specifying the `dockerfile` parameter. [See the `Dockerfile` reference for more information](https://docs.docker.com/engine/reference/builder/).  The Docker daemon performs a preliminary validation of the `Dockerfile` before starting the build, and returns an error if the syntax is incorrect. After that, each instruction is run one-by-one until the ID of the new image is output.  The build is canceled if the client drops the connection by quitting or being killed.
pub fn image_build(
    configuration: &configuration::Configuration,
    params: ImageBuildParams,
) -> Result<(), Error<ImageBuildError>> {
    let uri_str = format!("{}/build", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.dockerfile {
        req_builder = req_builder.query(&[("dockerfile", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.t {
        req_builder = req_builder.query(&[("t", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.extrahosts {
        req_builder = req_builder.query(&[("extrahosts", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.remote {
        req_builder = req_builder.query(&[("remote", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.q {
        req_builder = req_builder.query(&[("q", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.nocache {
        req_builder = req_builder.query(&[("nocache", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cachefrom {
        req_builder = req_builder.query(&[("cachefrom", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.pull {
        req_builder = req_builder.query(&[("pull", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.rm {
        req_builder = req_builder.query(&[("rm", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.forcerm {
        req_builder = req_builder.query(&[("forcerm", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.memory {
        req_builder = req_builder.query(&[("memory", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.memswap {
        req_builder = req_builder.query(&[("memswap", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cpushares {
        req_builder = req_builder.query(&[("cpushares", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cpusetcpus {
        req_builder = req_builder.query(&[("cpusetcpus", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cpuperiod {
        req_builder = req_builder.query(&[("cpuperiod", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.cpuquota {
        req_builder = req_builder.query(&[("cpuquota", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.buildargs {
        req_builder = req_builder.query(&[("buildargs", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.shmsize {
        req_builder = req_builder.query(&[("shmsize", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.squash {
        req_builder = req_builder.query(&[("squash", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.labels {
        req_builder = req_builder.query(&[("labels", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.networkmode {
        req_builder = req_builder.query(&[("networkmode", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.target {
        req_builder = req_builder.query(&[("target", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.outputs {
        req_builder = req_builder.query(&[("outputs", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.version {
        req_builder = req_builder.query(&[("version", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.content_type {
        req_builder = req_builder.header("Content-type", param_value.to_string());
    }
    if let Some(param_value) = params.x_registry_config {
        req_builder = req_builder.header("X-Registry-Config", param_value.to_string());
    }
    req_builder = req_builder.body(params.input_stream);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ImageBuildError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn image_commit(
    configuration: &configuration::Configuration,
    params: ImageCommitParams,
) -> Result<models::IdResponse, Error<ImageCommitError>> {
    let uri_str = format!("{}/commit", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.container {
        req_builder = req_builder.query(&[("container", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.repo {
        req_builder = req_builder.query(&[("repo", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.tag {
        req_builder = req_builder.query(&[("tag", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.comment {
        req_builder = req_builder.query(&[("comment", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.author {
        req_builder = req_builder.query(&[("author", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.pause {
        req_builder = req_builder.query(&[("pause", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.changes {
        req_builder = req_builder.query(&[("changes", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.container_config);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageCommitError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Pull or import an image.
pub fn image_create(
    configuration: &configuration::Configuration,
    params: ImageCreateParams,
) -> Result<(), Error<ImageCreateError>> {
    let uri_str = format!("{}/images/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.from_image {
        req_builder = req_builder.query(&[("fromImage", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.from_src {
        req_builder = req_builder.query(&[("fromSrc", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.repo {
        req_builder = req_builder.query(&[("repo", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.tag {
        req_builder = req_builder.query(&[("tag", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.message {
        req_builder = req_builder.query(&[("message", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.changes {
        req_builder = match "csv" {
            "multi" => {
                req_builder
                    .query(
                        &param_value
                            .into_iter()
                            .map(|p| ("changes".to_owned(), p.to_string()))
                            .collect::<Vec<(std::string::String, std::string::String)>>(),
                    )
            }
            _ => {
                req_builder
                    .query(
                        &[
                            (
                                "changes",
                                &param_value
                                    .into_iter()
                                    .map(|p| p.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",")
                                    .to_string(),
                            ),
                        ],
                    )
            }
        };
    }
    if let Some(ref param_value) = params.platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_registry_auth {
        req_builder = req_builder.header("X-Registry-Auth", param_value.to_string());
    }
    req_builder = req_builder.json(&params.input_image);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ImageCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Remove an image, along with any untagged parent images that were referenced by that image.  Images can't be removed if they have descendant images, are being used by a running container or are being used by a build.
pub fn image_delete(
    configuration: &configuration::Configuration,
    params: ImageDeleteParams,
) -> Result<Vec<models::ImageDeleteResponseItem>, Error<ImageDeleteError>> {
    let uri_str = format!(
        "{}/images/{name}", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref param_value) = params.force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.noprune {
        req_builder = req_builder.query(&[("noprune", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Get a tarball containing all images and metadata for a repository.  If `name` is a specific name and tag (e.g. `ubuntu:latest`), then only that image (and its parents) are returned. If `name` is an image ID, similarly only that image (and its parents) are returned, but with the exclusion of the `repositories` file in the tarball, as there were no image names referenced.  ### Image tarball format  An image tarball contains one directory per image layer (named using its long ID), each containing these files:  - `VERSION`: currently `1.0` - the file format version - `json`: detailed layer information, similar to `docker inspect layer_id` - `layer.tar`: A tarfile containing the filesystem changes in this layer  The `layer.tar` file contains `aufs` style `.wh..wh.aufs` files and directories for storing attribute changes and deletions.  If the tarball defines a repository, the tarball should also include a `repositories` file at the root that contains a list of repository and tag names mapped to layer IDs.  ```json {   \"hello-world\": {     \"latest\": \"565a9d68a73f6706862bfe8409a7f659776d4d60a8d096eb4a3cbce6999cc2a1\"   } } ```
pub fn image_get(
    configuration: &configuration::Configuration,
    params: ImageGetParams,
) -> Result<reqwest::blocking::Response, Error<ImageGetError>> {
    let uri_str = format!(
        "{}/images/{name}/get", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageGetError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Get a tarball containing all images and metadata for several image repositories.  For each value of the `names` parameter: if it is a specific name and tag (e.g. `ubuntu:latest`), then only that image (and its parents) are returned; if it is an image ID, similarly only that image (and its parents) are returned and there would be no names referenced in the 'repositories' file for this image ID.  For details on the format, see the [export image endpoint](#operation/ImageGet).
pub fn image_get_all(
    configuration: &configuration::Configuration,
    params: ImageGetAllParams,
) -> Result<reqwest::blocking::Response, Error<ImageGetAllError>> {
    let uri_str = format!("{}/images/get", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.names {
        req_builder = match "csv" {
            "multi" => {
                req_builder
                    .query(
                        &param_value
                            .into_iter()
                            .map(|p| ("names".to_owned(), p.to_string()))
                            .collect::<Vec<(std::string::String, std::string::String)>>(),
                    )
            }
            _ => {
                req_builder
                    .query(
                        &[
                            (
                                "names",
                                &param_value
                                    .into_iter()
                                    .map(|p| p.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",")
                                    .to_string(),
                            ),
                        ],
                    )
            }
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageGetAllError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Return parent layers of an image.
pub fn image_history(
    configuration: &configuration::Configuration,
    params: ImageHistoryParams,
) -> Result<Vec<models::HistoryResponseItem>, Error<ImageHistoryError>> {
    let uri_str = format!(
        "{}/images/{name}/history", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageHistoryError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Return low-level information about an image.
pub fn image_inspect(
    configuration: &configuration::Configuration,
    params: ImageInspectParams,
) -> Result<models::ImageInspect, Error<ImageInspectError>> {
    let uri_str = format!(
        "{}/images/{name}/json", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Returns a list of images on the server. Note that it uses a different, smaller representation of an image than inspecting a single image.
pub fn image_list(
    configuration: &configuration::Configuration,
    params: ImageListParams,
) -> Result<Vec<models::ImageSummary>, Error<ImageListError>> {
    let uri_str = format!("{}/images/json", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.all {
        req_builder = req_builder.query(&[("all", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.shared_size {
        req_builder = req_builder.query(&[("shared-size", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.digests {
        req_builder = req_builder.query(&[("digests", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.manifests {
        req_builder = req_builder.query(&[("manifests", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Load a set of images and tags into a repository.  For details on the format, see the [export image endpoint](#operation/ImageGet).
pub fn image_load(
    configuration: &configuration::Configuration,
    params: ImageLoadParams,
) -> Result<(), Error<ImageLoadError>> {
    let uri_str = format!("{}/images/load", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.quiet {
        req_builder = req_builder.query(&[("quiet", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.body(params.images_tarball);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ImageLoadError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn image_prune(
    configuration: &configuration::Configuration,
    params: ImagePruneParams,
) -> Result<models::ImagePruneResponse, Error<ImagePruneError>> {
    let uri_str = format!("{}/images/prune", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ImagePruneError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Push an image to a registry.  If you wish to push an image on to a private registry, that image must already have a tag which references the registry. For example, `registry.example.com/myimage:latest`.  The push is cancelled if the HTTP connection is closed.
pub fn image_push(
    configuration: &configuration::Configuration,
    params: ImagePushParams,
) -> Result<(), Error<ImagePushError>> {
    let uri_str = format!(
        "{}/images/{name}/push", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.tag {
        req_builder = req_builder.query(&[("tag", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder
        .header("X-Registry-Auth", params.x_registry_auth.to_string());
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ImagePushError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Search for an image on Docker Hub.
pub fn image_search(
    configuration: &configuration::Configuration,
    params: ImageSearchParams,
) -> Result<Vec<models::ImageSearchResponseItem>, Error<ImageSearchError>> {
    let uri_str = format!("{}/images/search", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    req_builder = req_builder.query(&[("term", &params.term.to_string())]);
    if let Some(ref param_value) = params.limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ImageSearchError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Tag an image so that it becomes part of a repository.
pub fn image_tag(
    configuration: &configuration::Configuration,
    params: ImageTagParams,
) -> Result<(), Error<ImageTagError>> {
    let uri_str = format!(
        "{}/images/{name}/tag", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.repo {
        req_builder = req_builder.query(&[("repo", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.tag {
        req_builder = req_builder.query(&[("tag", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ImageTagError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`network_connect`]
#[derive(Clone, Debug)]
pub struct NetworkConnectParams {
    /// Network ID or name
    pub id: String,
    pub container: models::NetworkConnectRequest,
}
/// struct for passing parameters to the method [`network_create`]
#[derive(Clone, Debug)]
pub struct NetworkCreateParams {
    /// Network configuration
    pub network_config: models::NetworkCreateRequest,
}
/// struct for passing parameters to the method [`network_delete`]
#[derive(Clone, Debug)]
pub struct NetworkDeleteParams {
    /// Network ID or name
    pub id: String,
}
/// struct for passing parameters to the method [`network_disconnect`]
#[derive(Clone, Debug)]
pub struct NetworkDisconnectParams {
    /// Network ID or name
    pub id: String,
    pub container: models::NetworkDisconnectRequest,
}
/// struct for passing parameters to the method [`network_inspect`]
#[derive(Clone, Debug)]
pub struct NetworkInspectParams {
    /// Network ID or name
    pub id: String,
    /// Detailed inspect output for troubleshooting
    pub verbose: Option<bool>,
    /// Filter the network by scope (swarm, global, or local)
    pub scope: Option<String>,
}
/// struct for passing parameters to the method [`network_list`]
#[derive(Clone, Debug)]
pub struct NetworkListParams {
    /// JSON encoded value of the filters (a `map[string][]string`) to process on the networks list.  Available filters:  - `dangling=<boolean>` When set to `true` (or `1`), returns all    networks that are not in use by a container. When set to `false`    (or `0`), only networks that are in use by one or more    containers are returned. - `driver=<driver-name>` Matches a network's driver. - `id=<network-id>` Matches all or part of a network ID. - `label=<key>` or `label=<key>=<value>` of a network label. - `name=<network-name>` Matches all or part of a network name. - `scope=[\"swarm\"|\"global\"|\"local\"]` Filters networks by scope (`swarm`, `global`, or `local`). - `type=[\"custom\"|\"builtin\"]` Filters networks by type. The `custom` keyword returns all user-defined networks.
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`network_prune`]
#[derive(Clone, Debug)]
pub struct NetworkPruneParams {
    /// Filters to process on the prune list, encoded as JSON (a `map[string][]string`).  Available filters: - `until=<timestamp>` Prune networks created before this timestamp. The `<timestamp>` can be Unix timestamps, date formatted timestamps, or Go duration strings (e.g. `10m`, `1h30m`) computed relative to the daemon machine’s time. - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune networks with (or without, in case `label!=...` is used) the specified labels.
    pub filters: Option<String>,
}
/// struct for typed errors of method [`network_connect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkConnectError {
    Status400(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`network_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkCreateError {
    Status400(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`network_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkDeleteError {
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`network_disconnect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkDisconnectError {
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`network_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`network_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkListError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`network_prune`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NetworkPruneError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// The network must be either a local-scoped network or a swarm-scoped network with the `attachable` option set. A network cannot be re-attached to a running container
pub fn network_connect(
    configuration: &configuration::Configuration,
    params: NetworkConnectParams,
) -> Result<(), Error<NetworkConnectError>> {
    let uri_str = format!(
        "{}/networks/{id}/connect", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.container);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<NetworkConnectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn network_create(
    configuration: &configuration::Configuration,
    params: NetworkCreateParams,
) -> Result<models::NetworkCreateResponse, Error<NetworkCreateError>> {
    let uri_str = format!("{}/networks/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.network_config);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<NetworkCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn network_delete(
    configuration: &configuration::Configuration,
    params: NetworkDeleteParams,
) -> Result<(), Error<NetworkDeleteError>> {
    let uri_str = format!(
        "{}/networks/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<NetworkDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn network_disconnect(
    configuration: &configuration::Configuration,
    params: NetworkDisconnectParams,
) -> Result<(), Error<NetworkDisconnectError>> {
    let uri_str = format!(
        "{}/networks/{id}/disconnect", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.container);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<NetworkDisconnectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn network_inspect(
    configuration: &configuration::Configuration,
    params: NetworkInspectParams,
) -> Result<models::Network, Error<NetworkInspectError>> {
    let uri_str = format!(
        "{}/networks/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.verbose {
        req_builder = req_builder.query(&[("verbose", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.scope {
        req_builder = req_builder.query(&[("scope", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<NetworkInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Returns a list of networks. For details on the format, see the [network inspect endpoint](#operation/NetworkInspect).  Note that it uses a different, smaller representation of a network than inspecting a single network. For example, the list of containers attached to the network is not propagated in API versions 1.28 and up.
pub fn network_list(
    configuration: &configuration::Configuration,
    params: NetworkListParams,
) -> Result<Vec<models::Network>, Error<NetworkListError>> {
    let uri_str = format!("{}/networks", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<NetworkListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn network_prune(
    configuration: &configuration::Configuration,
    params: NetworkPruneParams,
) -> Result<models::NetworkPruneResponse, Error<NetworkPruneError>> {
    let uri_str = format!("{}/networks/prune", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<NetworkPruneError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`node_delete`]
#[derive(Clone, Debug)]
pub struct NodeDeleteParams {
    /// The ID or name of the node
    pub id: String,
    /// Force remove a node from the swarm
    pub force: Option<bool>,
}
/// struct for passing parameters to the method [`node_inspect`]
#[derive(Clone, Debug)]
pub struct NodeInspectParams {
    /// The ID or name of the node
    pub id: String,
}
/// struct for passing parameters to the method [`node_list`]
#[derive(Clone, Debug)]
pub struct NodeListParams {
    /// Filters to process on the nodes list, encoded as JSON (a `map[string][]string`).  Available filters: - `id=<node id>` - `label=<engine label>` - `membership=`(`accepted`|`pending`)` - `name=<node name>` - `node.label=<node label>` - `role=`(`manager`|`worker`)`
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`node_update`]
#[derive(Clone, Debug)]
pub struct NodeUpdateParams {
    /// The ID of the node
    pub id: String,
    /// The version number of the node object being updated. This is required to avoid conflicting writes.
    pub version: i64,
    pub body: Option<models::NodeSpec>,
}
/// struct for typed errors of method [`node_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeDeleteError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`node_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`node_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeListError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`node_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeUpdateError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn node_delete(
    configuration: &configuration::Configuration,
    params: NodeDeleteParams,
) -> Result<(), Error<NodeDeleteError>> {
    let uri_str = format!(
        "{}/nodes/{id}", configuration.base_path, id = crate ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref param_value) = params.force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<NodeDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn node_inspect(
    configuration: &configuration::Configuration,
    params: NodeInspectParams,
) -> Result<models::Node, Error<NodeInspectError>> {
    let uri_str = format!(
        "{}/nodes/{id}", configuration.base_path, id = crate ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<NodeInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn node_list(
    configuration: &configuration::Configuration,
    params: NodeListParams,
) -> Result<Vec<models::Node>, Error<NodeListError>> {
    let uri_str = format!("{}/nodes", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<NodeListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn node_update(
    configuration: &configuration::Configuration,
    params: NodeUpdateParams,
) -> Result<(), Error<NodeUpdateError>> {
    let uri_str = format!(
        "{}/nodes/{id}/update", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("version", &params.version.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<NodeUpdateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`get_plugin_privileges`]
#[derive(Clone, Debug)]
pub struct GetPluginPrivilegesParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub remote: String,
}
/// struct for passing parameters to the method [`plugin_create`]
#[derive(Clone, Debug)]
pub struct PluginCreateParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
    /// Path to tar containing plugin rootfs and manifest
    pub tar_context: Option<std::path::PathBuf>,
}
/// struct for passing parameters to the method [`plugin_delete`]
#[derive(Clone, Debug)]
pub struct PluginDeleteParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
    /// Disable the plugin before removing. This may result in issues if the plugin is in use by a container.
    pub force: Option<bool>,
}
/// struct for passing parameters to the method [`plugin_disable`]
#[derive(Clone, Debug)]
pub struct PluginDisableParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
    /// Force disable a plugin even if still in use.
    pub force: Option<bool>,
}
/// struct for passing parameters to the method [`plugin_enable`]
#[derive(Clone, Debug)]
pub struct PluginEnableParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
    /// Set the HTTP client timeout (in seconds)
    pub timeout: Option<i32>,
}
/// struct for passing parameters to the method [`plugin_inspect`]
#[derive(Clone, Debug)]
pub struct PluginInspectParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
}
/// struct for passing parameters to the method [`plugin_list`]
#[derive(Clone, Debug)]
pub struct PluginListParams {
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the plugin list.  Available filters:  - `capability=<capability name>` - `enable=<true>|<false>`
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`plugin_pull`]
#[derive(Clone, Debug)]
pub struct PluginPullParams {
    /// Remote reference for plugin to install.  The `:latest` tag is optional, and is used as the default if omitted.
    pub remote: String,
    /// Local name for the pulled plugin.  The `:latest` tag is optional, and is used as the default if omitted.
    pub name: Option<String>,
    /// A base64url-encoded auth configuration to use when pulling a plugin from a registry.  Refer to the [authentication section](#section/Authentication) for details.
    pub x_registry_auth: Option<String>,
    pub body: Option<Vec<models::PluginPrivilege>>,
}
/// struct for passing parameters to the method [`plugin_push`]
#[derive(Clone, Debug)]
pub struct PluginPushParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
}
/// struct for passing parameters to the method [`plugin_set`]
#[derive(Clone, Debug)]
pub struct PluginSetParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
    pub body: Option<Vec<String>>,
}
/// struct for passing parameters to the method [`plugin_upgrade`]
#[derive(Clone, Debug)]
pub struct PluginUpgradeParams {
    /// The name of the plugin. The `:latest` tag is optional, and is the default if omitted.
    pub name: String,
    /// Remote reference to upgrade to.  The `:latest` tag is optional, and is used as the default if omitted.
    pub remote: String,
    /// A base64url-encoded auth configuration to use when pulling a plugin from a registry.  Refer to the [authentication section](#section/Authentication) for details.
    pub x_registry_auth: Option<String>,
    pub body: Option<Vec<models::PluginPrivilege>>,
}
/// struct for typed errors of method [`get_plugin_privileges`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetPluginPrivilegesError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginCreateError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginDeleteError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_disable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginDisableError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_enable`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginEnableError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginListError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_pull`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginPullError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_push`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginPushError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_set`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginSetError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`plugin_upgrade`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PluginUpgradeError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn get_plugin_privileges(
    configuration: &configuration::Configuration,
    params: GetPluginPrivilegesParams,
) -> Result<Vec<models::PluginPrivilege>, Error<GetPluginPrivilegesError>> {
    let uri_str = format!("{}/plugins/privileges", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    req_builder = req_builder.query(&[("remote", &params.remote.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<GetPluginPrivilegesError> = serde_json::from_str(&content)
            .ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn plugin_create(
    configuration: &configuration::Configuration,
    params: PluginCreateParams,
) -> Result<(), Error<PluginCreateError>> {
    let uri_str = format!("{}/plugins/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("name", &params.name.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.body(params.tar_context);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PluginCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn plugin_delete(
    configuration: &configuration::Configuration,
    params: PluginDeleteParams,
) -> Result<models::Plugin, Error<PluginDeleteError>> {
    let uri_str = format!(
        "{}/plugins/{name}", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref param_value) = params.force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<PluginDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn plugin_disable(
    configuration: &configuration::Configuration,
    params: PluginDisableParams,
) -> Result<(), Error<PluginDisableError>> {
    let uri_str = format!(
        "{}/plugins/{name}/disable", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PluginDisableError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn plugin_enable(
    configuration: &configuration::Configuration,
    params: PluginEnableParams,
) -> Result<(), Error<PluginEnableError>> {
    let uri_str = format!(
        "{}/plugins/{name}/enable", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.timeout {
        req_builder = req_builder.query(&[("timeout", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PluginEnableError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn plugin_inspect(
    configuration: &configuration::Configuration,
    params: PluginInspectParams,
) -> Result<models::Plugin, Error<PluginInspectError>> {
    let uri_str = format!(
        "{}/plugins/{name}/json", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<PluginInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Returns information about installed plugins.
pub fn plugin_list(
    configuration: &configuration::Configuration,
    params: PluginListParams,
) -> Result<Vec<models::Plugin>, Error<PluginListError>> {
    let uri_str = format!("{}/plugins", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<PluginListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Pulls and installs a plugin. After the plugin is installed, it can be enabled using the [`POST /plugins/{name}/enable` endpoint](#operation/PostPluginsEnable).
pub fn plugin_pull(
    configuration: &configuration::Configuration,
    params: PluginPullParams,
) -> Result<(), Error<PluginPullError>> {
    let uri_str = format!("{}/plugins/pull", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("remote", &params.remote.to_string())]);
    if let Some(ref param_value) = params.name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_registry_auth {
        req_builder = req_builder.header("X-Registry-Auth", param_value.to_string());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PluginPullError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Push a plugin to the registry.
pub fn plugin_push(
    configuration: &configuration::Configuration,
    params: PluginPushParams,
) -> Result<(), Error<PluginPushError>> {
    let uri_str = format!(
        "{}/plugins/{name}/push", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PluginPushError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn plugin_set(
    configuration: &configuration::Configuration,
    params: PluginSetParams,
) -> Result<(), Error<PluginSetError>> {
    let uri_str = format!(
        "{}/plugins/{name}/set", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PluginSetError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn plugin_upgrade(
    configuration: &configuration::Configuration,
    params: PluginUpgradeParams,
) -> Result<(), Error<PluginUpgradeError>> {
    let uri_str = format!(
        "{}/plugins/{name}/upgrade", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("remote", &params.remote.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_registry_auth {
        req_builder = req_builder.header("X-Registry-Auth", param_value.to_string());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<PluginUpgradeError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`secret_create`]
#[derive(Clone, Debug)]
pub struct SecretCreateParams {
    pub body: Option<models::SecretCreateRequest>,
}
/// struct for passing parameters to the method [`secret_delete`]
#[derive(Clone, Debug)]
pub struct SecretDeleteParams {
    /// ID of the secret
    pub id: String,
}
/// struct for passing parameters to the method [`secret_inspect`]
#[derive(Clone, Debug)]
pub struct SecretInspectParams {
    /// ID of the secret
    pub id: String,
}
/// struct for passing parameters to the method [`secret_list`]
#[derive(Clone, Debug)]
pub struct SecretListParams {
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the secrets list.  Available filters:  - `id=<secret id>` - `label=<key> or label=<key>=value` - `name=<secret name>` - `names=<secret name>`
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`secret_update`]
#[derive(Clone, Debug)]
pub struct SecretUpdateParams {
    /// The ID or name of the secret
    pub id: String,
    /// The version number of the secret object being updated. This is required to avoid conflicting writes.
    pub version: i64,
    /// The spec of the secret to update. Currently, only the Labels field can be updated. All other fields must remain unchanged from the [SecretInspect endpoint](#operation/SecretInspect) response values.
    pub body: Option<models::SecretSpec>,
}
/// struct for typed errors of method [`secret_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretCreateError {
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`secret_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretDeleteError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`secret_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`secret_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretListError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`secret_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SecretUpdateError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn secret_create(
    configuration: &configuration::Configuration,
    params: SecretCreateParams,
) -> Result<models::IdResponse, Error<SecretCreateError>> {
    let uri_str = format!("{}/secrets/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SecretCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn secret_delete(
    configuration: &configuration::Configuration,
    params: SecretDeleteParams,
) -> Result<(), Error<SecretDeleteError>> {
    let uri_str = format!(
        "{}/secrets/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<SecretDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn secret_inspect(
    configuration: &configuration::Configuration,
    params: SecretInspectParams,
) -> Result<models::Secret, Error<SecretInspectError>> {
    let uri_str = format!(
        "{}/secrets/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SecretInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn secret_list(
    configuration: &configuration::Configuration,
    params: SecretListParams,
) -> Result<Vec<models::Secret>, Error<SecretListError>> {
    let uri_str = format!("{}/secrets", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SecretListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn secret_update(
    configuration: &configuration::Configuration,
    params: SecretUpdateParams,
) -> Result<(), Error<SecretUpdateError>> {
    let uri_str = format!(
        "{}/secrets/{id}/update", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("version", &params.version.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<SecretUpdateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`service_create`]
#[derive(Clone, Debug)]
pub struct ServiceCreateParams {
    pub body: models::ServiceCreateRequest,
    /// A base64url-encoded auth configuration for pulling from private registries.  Refer to the [authentication section](#section/Authentication) for details.
    pub x_registry_auth: Option<String>,
}
/// struct for passing parameters to the method [`service_delete`]
#[derive(Clone, Debug)]
pub struct ServiceDeleteParams {
    /// ID or name of service.
    pub id: String,
}
/// struct for passing parameters to the method [`service_inspect`]
#[derive(Clone, Debug)]
pub struct ServiceInspectParams {
    /// ID or name of service.
    pub id: String,
    /// Fill empty fields with default values.
    pub insert_defaults: Option<bool>,
}
/// struct for passing parameters to the method [`service_list`]
#[derive(Clone, Debug)]
pub struct ServiceListParams {
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the services list.  Available filters:  - `id=<service id>` - `label=<service label>` - `mode=[\"replicated\"|\"global\"]` - `name=<service name>`
    pub filters: Option<String>,
    /// Include service status, with count of running and desired tasks.
    pub status: Option<bool>,
}
/// struct for passing parameters to the method [`service_logs`]
#[derive(Clone, Debug)]
pub struct ServiceLogsParams {
    /// ID or name of the service
    pub id: String,
    /// Show service context and extra details provided to logs.
    pub details: Option<bool>,
    /// Keep connection after returning logs.
    pub follow: Option<bool>,
    /// Return logs from `stdout`
    pub stdout: Option<bool>,
    /// Return logs from `stderr`
    pub stderr: Option<bool>,
    /// Only return logs since this time, as a UNIX timestamp
    pub since: Option<i32>,
    /// Add timestamps to every log line
    pub timestamps: Option<bool>,
    /// Only return this number of log lines from the end of the logs. Specify as an integer or `all` to output all log lines.
    pub tail: Option<String>,
}
/// struct for passing parameters to the method [`service_update`]
#[derive(Clone, Debug)]
pub struct ServiceUpdateParams {
    /// ID or name of service.
    pub id: String,
    /// The version number of the service object being updated. This is required to avoid conflicting writes. This version number should be the value as currently set on the service *before* the update. You can find the current version by calling `GET /services/{id}`
    pub version: i32,
    pub body: models::ServiceUpdateRequest,
    /// If the `X-Registry-Auth` header is not specified, this parameter indicates where to find registry authorization credentials.
    pub registry_auth_from: Option<String>,
    /// Set to this parameter to `previous` to cause a server-side rollback to the previous service spec. The supplied spec will be ignored in this case.
    pub rollback: Option<String>,
    /// A base64url-encoded auth configuration for pulling from private registries.  Refer to the [authentication section](#section/Authentication) for details.
    pub x_registry_auth: Option<String>,
}
/// struct for typed errors of method [`service_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceCreateError {
    Status400(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`service_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceDeleteError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`service_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`service_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceListError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`service_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceLogsError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`service_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServiceUpdateError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn service_create(
    configuration: &configuration::Configuration,
    params: ServiceCreateParams,
) -> Result<models::ServiceCreateResponse, Error<ServiceCreateError>> {
    let uri_str = format!("{}/services/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_registry_auth {
        req_builder = req_builder.header("X-Registry-Auth", param_value.to_string());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ServiceCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn service_delete(
    configuration: &configuration::Configuration,
    params: ServiceDeleteParams,
) -> Result<(), Error<ServiceDeleteError>> {
    let uri_str = format!(
        "{}/services/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<ServiceDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn service_inspect(
    configuration: &configuration::Configuration,
    params: ServiceInspectParams,
) -> Result<models::Service, Error<ServiceInspectError>> {
    let uri_str = format!(
        "{}/services/{id}", configuration.base_path, id = crate ::apis::urlencode(params
        .id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.insert_defaults {
        req_builder = req_builder.query(&[("insertDefaults", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ServiceInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn service_list(
    configuration: &configuration::Configuration,
    params: ServiceListParams,
) -> Result<Vec<models::Service>, Error<ServiceListError>> {
    let uri_str = format!("{}/services", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.status {
        req_builder = req_builder.query(&[("status", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ServiceListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Get `stdout` and `stderr` logs from a service. See also [`/containers/{id}/logs`](#operation/ContainerLogs).  **Note**: This endpoint works only for services with the `local`, `json-file` or `journald` logging drivers.
pub fn service_logs(
    configuration: &configuration::Configuration,
    params: ServiceLogsParams,
) -> Result<reqwest::blocking::Response, Error<ServiceLogsError>> {
    let uri_str = format!(
        "{}/services/{id}/logs", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.details {
        req_builder = req_builder.query(&[("details", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stdout {
        req_builder = req_builder.query(&[("stdout", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stderr {
        req_builder = req_builder.query(&[("stderr", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.timestamps {
        req_builder = req_builder.query(&[("timestamps", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.tail {
        req_builder = req_builder.query(&[("tail", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text()?;
        let entity: Option<ServiceLogsError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn service_update(
    configuration: &configuration::Configuration,
    params: ServiceUpdateParams,
) -> Result<models::ServiceUpdateResponse, Error<ServiceUpdateError>> {
    let uri_str = format!(
        "{}/services/{id}/update", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("version", &params.version.to_string())]);
    if let Some(ref param_value) = params.registry_auth_from {
        req_builder = req_builder
            .query(&[("registryAuthFrom", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.rollback {
        req_builder = req_builder.query(&[("rollback", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(param_value) = params.x_registry_auth {
        req_builder = req_builder.header("X-Registry-Auth", param_value.to_string());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<ServiceUpdateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for typed errors of method [`session`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionError {
    Status400(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// Start a new interactive session with a server. Session allows server to call back to the client for advanced capabilities.  ### Hijacking  This endpoint hijacks the HTTP connection to HTTP2 transport that allows the client to expose gPRC services on that connection.  For example, the client sends this request to upgrade the connection:  ``` POST /session HTTP/1.1 Upgrade: h2c Connection: Upgrade ```  The Docker daemon responds with a `101 UPGRADED` response follow with the raw stream:  ``` HTTP/1.1 101 UPGRADED Connection: Upgrade Upgrade: h2c ```
pub fn session(
    configuration: &configuration::Configuration,
) -> Result<(), Error<SessionError>> {
    let uri_str = format!("{}/session", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<SessionError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`swarm_init`]
#[derive(Clone, Debug)]
pub struct SwarmInitParams {
    pub body: models::SwarmInitRequest,
}
/// struct for passing parameters to the method [`swarm_join`]
#[derive(Clone, Debug)]
pub struct SwarmJoinParams {
    pub body: models::SwarmJoinRequest,
}
/// struct for passing parameters to the method [`swarm_leave`]
#[derive(Clone, Debug)]
pub struct SwarmLeaveParams {
    /// Force leave swarm, even if this is the last manager or that it will break the cluster.
    pub force: Option<bool>,
}
/// struct for passing parameters to the method [`swarm_unlock`]
#[derive(Clone, Debug)]
pub struct SwarmUnlockParams {
    pub body: models::SwarmUnlockRequest,
}
/// struct for passing parameters to the method [`swarm_update`]
#[derive(Clone, Debug)]
pub struct SwarmUpdateParams {
    /// The version number of the swarm object being updated. This is required to avoid conflicting writes.
    pub version: i64,
    pub body: models::SwarmSpec,
    /// Rotate the worker join token.
    pub rotate_worker_token: Option<bool>,
    /// Rotate the manager join token.
    pub rotate_manager_token: Option<bool>,
    /// Rotate the manager unlock key.
    pub rotate_manager_unlock_key: Option<bool>,
}
/// struct for typed errors of method [`swarm_init`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwarmInitError {
    Status400(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`swarm_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwarmInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`swarm_join`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwarmJoinError {
    Status400(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`swarm_leave`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwarmLeaveError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`swarm_unlock`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwarmUnlockError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`swarm_unlockkey`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwarmUnlockkeyError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`swarm_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwarmUpdateError {
    Status400(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn swarm_init(
    configuration: &configuration::Configuration,
    params: SwarmInitParams,
) -> Result<String, Error<SwarmInitError>> {
    let uri_str = format!("{}/swarm/init", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SwarmInitError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn swarm_inspect(
    configuration: &configuration::Configuration,
) -> Result<models::Swarm, Error<SwarmInspectError>> {
    let uri_str = format!("{}/swarm", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SwarmInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn swarm_join(
    configuration: &configuration::Configuration,
    params: SwarmJoinParams,
) -> Result<(), Error<SwarmJoinError>> {
    let uri_str = format!("{}/swarm/join", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<SwarmJoinError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn swarm_leave(
    configuration: &configuration::Configuration,
    params: SwarmLeaveParams,
) -> Result<(), Error<SwarmLeaveError>> {
    let uri_str = format!("{}/swarm/leave", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<SwarmLeaveError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn swarm_unlock(
    configuration: &configuration::Configuration,
    params: SwarmUnlockParams,
) -> Result<(), Error<SwarmUnlockError>> {
    let uri_str = format!("{}/swarm/unlock", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<SwarmUnlockError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn swarm_unlockkey(
    configuration: &configuration::Configuration,
) -> Result<models::UnlockKeyResponse, Error<SwarmUnlockkeyError>> {
    let uri_str = format!("{}/swarm/unlockkey", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SwarmUnlockkeyError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn swarm_update(
    configuration: &configuration::Configuration,
    params: SwarmUpdateParams,
) -> Result<(), Error<SwarmUpdateError>> {
    let uri_str = format!("{}/swarm/update", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    req_builder = req_builder.query(&[("version", &params.version.to_string())]);
    if let Some(ref param_value) = params.rotate_worker_token {
        req_builder = req_builder
            .query(&[("rotateWorkerToken", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.rotate_manager_token {
        req_builder = req_builder
            .query(&[("rotateManagerToken", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.rotate_manager_unlock_key {
        req_builder = req_builder
            .query(&[("rotateManagerUnlockKey", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<SwarmUpdateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`system_auth`]
#[derive(Clone, Debug)]
pub struct SystemAuthParams {
    /// Authentication to check
    pub auth_config: Option<models::AuthConfig>,
}
/// struct for passing parameters to the method [`system_data_usage`]
#[derive(Clone, Debug)]
pub struct SystemDataUsageParams {
    /// Object types, for which to compute and return data.
    pub r#type: Option<Vec<String>>,
}
/// struct for passing parameters to the method [`system_events`]
#[derive(Clone, Debug)]
pub struct SystemEventsParams {
    /// Show events created since this timestamp then stream new events.
    pub since: Option<String>,
    /// Show events created until this timestamp then stop streaming.
    pub until: Option<String>,
    /// A JSON encoded value of filters (a `map[string][]string`) to process on the event list. Available filters:  - `config=<string>` config name or ID - `container=<string>` container name or ID - `daemon=<string>` daemon name or ID - `event=<string>` event type - `image=<string>` image name or ID - `label=<string>` image or container label - `network=<string>` network name or ID - `node=<string>` node ID - `plugin`=<string> plugin name or ID - `scope`=<string> local or swarm - `secret=<string>` secret name or ID - `service=<string>` service name or ID - `type=<string>` object to filter by, one of `container`, `image`, `volume`, `network`, `daemon`, `plugin`, `node`, `service`, `secret` or `config` - `volume=<string>` volume name
    pub filters: Option<String>,
}
/// struct for typed errors of method [`system_auth`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemAuthError {
    Status401(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`system_data_usage`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemDataUsageError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`system_events`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemEventsError {
    Status400(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`system_info`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemInfoError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`system_ping`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemPingError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`system_ping_head`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemPingHeadError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`system_version`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemVersionError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// Validate credentials for a registry and, if available, get an identity token for accessing the registry without password.
pub fn system_auth(
    configuration: &configuration::Configuration,
    params: SystemAuthParams,
) -> Result<models::SystemAuthResponse, Error<SystemAuthError>> {
    let uri_str = format!("{}/auth", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.auth_config);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SystemAuthError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn system_data_usage(
    configuration: &configuration::Configuration,
    params: SystemDataUsageParams,
) -> Result<models::SystemDataUsageResponse, Error<SystemDataUsageError>> {
    let uri_str = format!("{}/system/df", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.r#type {
        req_builder = match "multi" {
            "multi" => {
                req_builder
                    .query(
                        &param_value
                            .into_iter()
                            .map(|p| ("type".to_owned(), p.to_string()))
                            .collect::<Vec<(std::string::String, std::string::String)>>(),
                    )
            }
            _ => {
                req_builder
                    .query(
                        &[
                            (
                                "type",
                                &param_value
                                    .into_iter()
                                    .map(|p| p.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",")
                                    .to_string(),
                            ),
                        ],
                    )
            }
        };
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SystemDataUsageError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Stream real-time events from the server.  Various objects within Docker report events when something happens to them.  Containers report these events: `attach`, `commit`, `copy`, `create`, `destroy`, `detach`, `die`, `exec_create`, `exec_detach`, `exec_start`, `exec_die`, `export`, `health_status`, `kill`, `oom`, `pause`, `rename`, `resize`, `restart`, `start`, `stop`, `top`, `unpause`, `update`, and `prune`  Images report these events: `create`, `delete`, `import`, `load`, `pull`, `push`, `save`, `tag`, `untag`, and `prune`  Volumes report these events: `create`, `mount`, `unmount`, `destroy`, and `prune`  Networks report these events: `create`, `connect`, `disconnect`, `destroy`, `update`, `remove`, and `prune`  The Docker daemon reports these events: `reload`  Services report these events: `create`, `update`, and `remove`  Nodes report these events: `create`, `update`, and `remove`  Secrets report these events: `create`, `update`, and `remove`  Configs report these events: `create`, `update`, and `remove`  The Builder reports `prune` events
pub fn system_events(
    configuration: &configuration::Configuration,
    params: SystemEventsParams,
) -> Result<models::EventMessage, Error<SystemEventsError>> {
    let uri_str = format!("{}/events", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.until {
        req_builder = req_builder.query(&[("until", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SystemEventsError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn system_info(
    configuration: &configuration::Configuration,
) -> Result<models::SystemInfo, Error<SystemInfoError>> {
    let uri_str = format!("{}/info", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SystemInfoError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// This is a dummy endpoint you can use to test if the server is accessible.
pub fn system_ping(
    configuration: &configuration::Configuration,
) -> Result<String, Error<SystemPingError>> {
    let uri_str = format!("{}/_ping", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SystemPingError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// This is a dummy endpoint you can use to test if the server is accessible.
pub fn system_ping_head(
    configuration: &configuration::Configuration,
) -> Result<String, Error<SystemPingHeadError>> {
    let uri_str = format!("{}/_ping", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::HEAD, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SystemPingHeadError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Returns the version of Docker that is running and various information about the system that Docker is running on.
pub fn system_version(
    configuration: &configuration::Configuration,
) -> Result<models::SystemVersion, Error<SystemVersionError>> {
    let uri_str = format!("{}/version", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<SystemVersionError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`task_inspect`]
#[derive(Clone, Debug)]
pub struct TaskInspectParams {
    /// ID of the task
    pub id: String,
}
/// struct for passing parameters to the method [`task_list`]
#[derive(Clone, Debug)]
pub struct TaskListParams {
    /// A JSON encoded value of the filters (a `map[string][]string`) to process on the tasks list.  Available filters:  - `desired-state=(running | shutdown | accepted)` - `id=<task id>` - `label=key` or `label=\"key=value\"` - `name=<task name>` - `node=<node id or name>` - `service=<service name>`
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`task_logs`]
#[derive(Clone, Debug)]
pub struct TaskLogsParams {
    /// ID of the task
    pub id: String,
    /// Show task context and extra details provided to logs.
    pub details: Option<bool>,
    /// Keep connection after returning logs.
    pub follow: Option<bool>,
    /// Return logs from `stdout`
    pub stdout: Option<bool>,
    /// Return logs from `stderr`
    pub stderr: Option<bool>,
    /// Only return logs since this time, as a UNIX timestamp
    pub since: Option<i32>,
    /// Add timestamps to every log line
    pub timestamps: Option<bool>,
    /// Only return this number of log lines from the end of the logs. Specify as an integer or `all` to output all log lines.
    pub tail: Option<String>,
}
/// struct for typed errors of method [`task_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`task_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskListError {
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`task_logs`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskLogsError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn task_inspect(
    configuration: &configuration::Configuration,
    params: TaskInspectParams,
) -> Result<models::Task, Error<TaskInspectError>> {
    let uri_str = format!(
        "{}/tasks/{id}", configuration.base_path, id = crate ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<TaskInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn task_list(
    configuration: &configuration::Configuration,
    params: TaskListParams,
) -> Result<Vec<models::Task>, Error<TaskListError>> {
    let uri_str = format!("{}/tasks", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<TaskListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Get `stdout` and `stderr` logs from a task. See also [`/containers/{id}/logs`](#operation/ContainerLogs).  **Note**: This endpoint works only for services with the `local`, `json-file` or `journald` logging drivers.
pub fn task_logs(
    configuration: &configuration::Configuration,
    params: TaskLogsParams,
) -> Result<reqwest::blocking::Response, Error<TaskLogsError>> {
    let uri_str = format!(
        "{}/tasks/{id}/logs", configuration.base_path, id = crate
        ::apis::urlencode(params.id)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.details {
        req_builder = req_builder.query(&[("details", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.follow {
        req_builder = req_builder.query(&[("follow", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stdout {
        req_builder = req_builder.query(&[("stdout", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.stderr {
        req_builder = req_builder.query(&[("stderr", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.since {
        req_builder = req_builder.query(&[("since", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.timestamps {
        req_builder = req_builder.query(&[("timestamps", &param_value.to_string())]);
    }
    if let Some(ref param_value) = params.tail {
        req_builder = req_builder.query(&[("tail", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(resp)
    } else {
        let content = resp.text()?;
        let entity: Option<TaskLogsError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};
/// struct for passing parameters to the method [`volume_create`]
#[derive(Clone, Debug)]
pub struct VolumeCreateParams {
    /// Volume configuration
    pub volume_config: models::VolumeCreateOptions,
}
/// struct for passing parameters to the method [`volume_delete`]
#[derive(Clone, Debug)]
pub struct VolumeDeleteParams {
    /// Volume name or ID
    pub name: String,
    /// Force the removal of the volume
    pub force: Option<bool>,
}
/// struct for passing parameters to the method [`volume_inspect`]
#[derive(Clone, Debug)]
pub struct VolumeInspectParams {
    /// Volume name or ID
    pub name: String,
}
/// struct for passing parameters to the method [`volume_list`]
#[derive(Clone, Debug)]
pub struct VolumeListParams {
    /// JSON encoded value of the filters (a `map[string][]string`) to process on the volumes list. Available filters:  - `dangling=<boolean>` When set to `true` (or `1`), returns all    volumes that are not in use by a container. When set to `false`    (or `0`), only volumes that are in use by one or more    containers are returned. - `driver=<volume-driver-name>` Matches volumes based on their driver. - `label=<key>` or `label=<key>:<value>` Matches volumes based on    the presence of a `label` alone or a `label` and a value. - `name=<volume-name>` Matches all or part of a volume name.
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`volume_prune`]
#[derive(Clone, Debug)]
pub struct VolumePruneParams {
    /// Filters to process on the prune list, encoded as JSON (a `map[string][]string`).  Available filters: - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or `label!=<key>=<value>`) Prune volumes with (or without, in case `label!=...` is used) the specified labels. - `all` (`all=true`) - Consider all (local) volumes for pruning and not just anonymous volumes.
    pub filters: Option<String>,
}
/// struct for passing parameters to the method [`volume_update`]
#[derive(Clone, Debug)]
pub struct VolumeUpdateParams {
    /// The name or ID of the volume
    pub name: String,
    /// The version number of the volume being updated. This is required to avoid conflicting writes. Found in the volume's `ClusterVolume` field.
    pub version: i64,
    /// The spec of the volume to update. Currently, only Availability may change. All other fields must remain unchanged.
    pub body: Option<models::VolumeUpdateRequest>,
}
/// struct for typed errors of method [`volume_create`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeCreateError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`volume_delete`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeDeleteError {
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`volume_inspect`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeInspectError {
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`volume_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeListError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`volume_prune`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumePruneError {
    Status500(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
/// struct for typed errors of method [`volume_update`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VolumeUpdateError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status500(models::ErrorResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
pub fn volume_create(
    configuration: &configuration::Configuration,
    params: VolumeCreateParams,
) -> Result<models::Volume, Error<VolumeCreateError>> {
    let uri_str = format!("{}/volumes/create", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.volume_config);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<VolumeCreateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
/// Instruct the driver to remove the volume.
pub fn volume_delete(
    configuration: &configuration::Configuration,
    params: VolumeDeleteParams,
) -> Result<(), Error<VolumeDeleteError>> {
    let uri_str = format!(
        "{}/volumes/{name}", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::DELETE, &uri_str);
    if let Some(ref param_value) = params.force {
        req_builder = req_builder.query(&[("force", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<VolumeDeleteError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn volume_inspect(
    configuration: &configuration::Configuration,
    params: VolumeInspectParams,
) -> Result<models::Volume, Error<VolumeInspectError>> {
    let uri_str = format!(
        "{}/volumes/{name}", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<VolumeInspectError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn volume_list(
    configuration: &configuration::Configuration,
    params: VolumeListParams,
) -> Result<models::VolumeListResponse, Error<VolumeListError>> {
    let uri_str = format!("{}/volumes", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<VolumeListError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn volume_prune(
    configuration: &configuration::Configuration,
    params: VolumePruneParams,
) -> Result<models::VolumePruneResponse, Error<VolumePruneError>> {
    let uri_str = format!("{}/volumes/prune", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    if let Some(ref param_value) = params.filters {
        req_builder = req_builder.query(&[("filters", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text()?;
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let content = resp.text()?;
        let entity: Option<VolumePruneError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
pub fn volume_update(
    configuration: &configuration::Configuration,
    params: VolumeUpdateParams,
) -> Result<(), Error<VolumeUpdateError>> {
    let uri_str = format!(
        "{}/volumes/{name}", configuration.base_path, name = crate
        ::apis::urlencode(params.name)
    );
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);
    req_builder = req_builder.query(&[("version", &params.version.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder
            .header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&params.body);
    let req = req_builder.build()?;
    let resp = configuration.client.execute(req)?;
    let status = resp.status();
    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let content = resp.text()?;
        let entity: Option<VolumeUpdateError> = serde_json::from_str(&content).ok();
        Err(
            Error::ResponseError(ResponseContent {
                status,
                content,
                entity,
            }),
        )
    }
}
use std::error;
use std::fmt;
#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}
#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}
impl<T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}
impl<T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(
            match self {
                Error::Reqwest(e) => e,
                Error::Serde(e) => e,
                Error::Io(e) => e,
                Error::ResponseError(_) => return None,
            },
        )
    }
}
impl<T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}
impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}
impl<T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}
pub fn parse_deep_object(
    prefix: &str,
    value: &serde_json::Value,
) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];
        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => {
                    params
                        .append(
                            &mut parse_deep_object(
                                &format!("{}[{}]", prefix, key),
                                value,
                            ),
                        )
                }
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params
                            .append(
                                &mut parse_deep_object(
                                    &format!("{}[{}][{}]", prefix, key, i),
                                    value,
                                ),
                            );
                    }
                }
                serde_json::Value::String(s) => {
                    params.push((format!("{}[{}]", prefix, key), s.clone()))
                }
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }
        return params;
    }
    unimplemented!("Only objects are supported with style=deepObject")
}
pub mod config_api;
pub mod container_api;
pub mod distribution_api;
pub mod exec_api;
pub mod image_api;
pub mod network_api;
pub mod node_api;
pub mod plugin_api;
pub mod secret_api;
pub mod service_api;
pub mod session_api;
pub mod swarm_api;
pub mod system_api;
pub mod task_api;
pub mod volume_api;
pub mod configuration;
#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::blocking::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}
pub type BasicAuth = (String, Option<String>);
#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}
impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}
impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost/v1.47".to_owned(),
            user_agent: Some("container-api-sync-0.1.0".to_owned()),
            client: reqwest::blocking::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
