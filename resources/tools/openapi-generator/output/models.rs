use crate::models;
use serde::{Deserialize, Serialize};
/// Address : Address represents an IPv4 or IPv6 IP address.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Address {
    /// IP address.
    #[serde(rename = "Addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
    /// Mask length of the IP address.
    #[serde(rename = "PrefixLen", skip_serializing_if = "Option::is_none")]
    pub prefix_len: Option<i32>,
}
impl Address {
    /// Address represents an IPv4 or IPv6 IP address.
    pub fn new() -> Address {
        Address {
            addr: None,
            prefix_len: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "serveraddress", skip_serializing_if = "Option::is_none")]
    pub serveraddress: Option<String>,
}
impl AuthConfig {
    pub fn new() -> AuthConfig {
        AuthConfig {
            username: None,
            password: None,
            email: None,
            serveraddress: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// BuildCache : BuildCache contains information about a build cache record.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildCache {
    /// Unique ID of the build cache record.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ID of the parent build cache record.  > **Deprecated**: This field is deprecated, and omitted if empty.
    #[serde(
        rename = "Parent",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub parent: Option<Option<String>>,
    /// List of parent build cache record IDs.
    #[serde(
        rename = "Parents",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub parents: Option<Option<Vec<String>>>,
    /// Cache record type.
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Description of the build-step that produced the build cache.
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Indicates if the build cache is in use.
    #[serde(rename = "InUse", skip_serializing_if = "Option::is_none")]
    pub in_use: Option<bool>,
    /// Indicates if the build cache is shared.
    #[serde(rename = "Shared", skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    /// Amount of disk space used by the build cache (in bytes).
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Date and time at which the build cache was created in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the build cache was last used in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(
        rename = "LastUsedAt",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_used_at: Option<Option<String>>,
    #[serde(rename = "UsageCount", skip_serializing_if = "Option::is_none")]
    pub usage_count: Option<i32>,
}
impl BuildCache {
    /// BuildCache contains information about a build cache record.
    pub fn new() -> BuildCache {
        BuildCache {
            id: None,
            parent: None,
            parents: None,
            r#type: None,
            description: None,
            in_use: None,
            shared: None,
            size: None,
            created_at: None,
            last_used_at: None,
            usage_count: None,
        }
    }
}
/// Cache record type.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Type {
    #[serde(rename = "internal")]
    Internal,
    #[serde(rename = "frontend")]
    Frontend,
    #[serde(rename = "source.local")]
    SourcePeriodLocal,
    #[serde(rename = "source.git.checkout")]
    SourcePeriodGitPeriodCheckout,
    #[serde(rename = "exec.cachemount")]
    ExecPeriodCachemount,
    #[serde(rename = "regular")]
    Regular,
}
impl Default for Type {
    fn default() -> Type {
        Self::Internal
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildInfo {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "stream", skip_serializing_if = "Option::is_none")]
    pub stream: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "errorDetail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<Box<models::ErrorDetail>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail", skip_serializing_if = "Option::is_none")]
    pub progress_detail: Option<Box<models::ProgressDetail>>,
    #[serde(rename = "aux", skip_serializing_if = "Option::is_none")]
    pub aux: Option<Box<models::ImageId>>,
}
impl BuildInfo {
    pub fn new() -> BuildInfo {
        BuildInfo {
            id: None,
            stream: None,
            error: None,
            error_detail: None,
            status: None,
            progress: None,
            progress_detail: None,
            aux: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BuildPruneResponse {
    #[serde(rename = "CachesDeleted", skip_serializing_if = "Option::is_none")]
    pub caches_deleted: Option<Vec<String>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed", skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
impl BuildPruneResponse {
    pub fn new() -> BuildPruneResponse {
        BuildPruneResponse {
            caches_deleted: None,
            space_reclaimed: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
/// ChangeType : Kind of change  Can be one of:  - `0`: Modified (\"C\") - `1`: Added (\"A\") - `2`: Deleted (\"D\")
/// Kind of change  Can be one of:  - `0`: Modified (\"C\") - `1`: Added (\"A\") - `2`: Deleted (\"D\")
#[repr(i64)]
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize_repr,
    Deserialize_repr
)]
pub enum ChangeType {
    Variant0 = 0,
    Variant1 = 1,
    Variant2 = 2,
}
impl std::fmt::Display for ChangeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "{}", match self { Self::Variant0 => "0", Self::Variant1 => "1",
            Self::Variant2 => "2", }
        )
    }
}
impl Default for ChangeType {
    fn default() -> ChangeType {
        Self::Variant0
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterInfo : ClusterInfo represents information about the swarm as is returned by the \"/info\" endpoint. Join-tokens are not included.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterInfo {
    /// The ID of the swarm.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    /// Date and time at which the swarm was initialised in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the swarm was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::SwarmSpec>>,
    #[serde(rename = "TLSInfo", skip_serializing_if = "Option::is_none")]
    pub tls_info: Option<Box<models::TlsInfo>>,
    /// Whether there is currently a root CA rotation in progress for the swarm
    #[serde(rename = "RootRotationInProgress", skip_serializing_if = "Option::is_none")]
    pub root_rotation_in_progress: Option<bool>,
    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. If no port is set or is set to 0, the default port (4789) is used.
    #[serde(rename = "DataPathPort", skip_serializing_if = "Option::is_none")]
    pub data_path_port: Option<i32>,
    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[serde(rename = "DefaultAddrPool", skip_serializing_if = "Option::is_none")]
    pub default_addr_pool: Option<Vec<String>>,
    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[serde(rename = "SubnetSize", skip_serializing_if = "Option::is_none")]
    pub subnet_size: Option<i32>,
}
impl ClusterInfo {
    /// ClusterInfo represents information about the swarm as is returned by the \"/info\" endpoint. Join-tokens are not included.
    pub fn new() -> ClusterInfo {
        ClusterInfo {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            spec: None,
            tls_info: None,
            root_rotation_in_progress: None,
            data_path_port: None,
            default_addr_pool: None,
            subnet_size: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterVolume : Options and information specific to, and only present on, Swarm CSI cluster volumes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolume {
    /// The Swarm ID of this volume. Because cluster volumes are Swarm objects, they have an ID, unlike non-cluster volumes. This ID can be used to refer to the Volume instead of the name.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::ClusterVolumeSpec>>,
    #[serde(rename = "Info", skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<models::ClusterVolumeInfo>>,
    /// The status of the volume as it pertains to its publishing and use on specific nodes
    #[serde(rename = "PublishStatus", skip_serializing_if = "Option::is_none")]
    pub publish_status: Option<Vec<models::ClusterVolumePublishStatusInner>>,
}
impl ClusterVolume {
    /// Options and information specific to, and only present on, Swarm CSI cluster volumes.
    pub fn new() -> ClusterVolume {
        ClusterVolume {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            spec: None,
            info: None,
            publish_status: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterVolumeInfo : Information about the global status of the volume.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumeInfo {
    /// The capacity of the volume in bytes. A value of 0 indicates that the capacity is unknown.
    #[serde(rename = "CapacityBytes", skip_serializing_if = "Option::is_none")]
    pub capacity_bytes: Option<i64>,
    /// A map of strings to strings returned from the storage plugin when the volume is created.
    #[serde(rename = "VolumeContext", skip_serializing_if = "Option::is_none")]
    pub volume_context: Option<std::collections::HashMap<String, String>>,
    /// The ID of the volume as returned by the CSI storage plugin. This is distinct from the volume's ID as provided by Docker. This ID is never used by the user when communicating with Docker to refer to this volume. If the ID is blank, then the Volume has not been successfully created in the plugin yet.
    #[serde(rename = "VolumeID", skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    /// The topology this volume is actually accessible from.
    #[serde(rename = "AccessibleTopology", skip_serializing_if = "Option::is_none")]
    pub accessible_topology: Option<Vec<std::collections::HashMap<String, String>>>,
}
impl ClusterVolumeInfo {
    /// Information about the global status of the volume.
    pub fn new() -> ClusterVolumeInfo {
        ClusterVolumeInfo {
            capacity_bytes: None,
            volume_context: None,
            volume_id: None,
            accessible_topology: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumePublishStatusInner {
    /// The ID of the Swarm node the volume is published on.
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// The published state of the volume. * `pending-publish` The volume should be published to this node, but the call to the controller plugin to do so has not yet been successfully completed. * `published` The volume is published successfully to the node. * `pending-node-unpublish` The volume should be unpublished from the node, and the manager is awaiting confirmation from the worker that it has done so. * `pending-controller-unpublish` The volume is successfully unpublished from the node, but has not yet been successfully unpublished on the controller.
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// A map of strings to strings returned by the CSI controller plugin when a volume is published.
    #[serde(rename = "PublishContext", skip_serializing_if = "Option::is_none")]
    pub publish_context: Option<std::collections::HashMap<String, String>>,
}
impl ClusterVolumePublishStatusInner {
    pub fn new() -> ClusterVolumePublishStatusInner {
        ClusterVolumePublishStatusInner {
            node_id: None,
            state: None,
            publish_context: None,
        }
    }
}
/// The published state of the volume. * `pending-publish` The volume should be published to this node, but the call to the controller plugin to do so has not yet been successfully completed. * `published` The volume is published successfully to the node. * `pending-node-unpublish` The volume should be unpublished from the node, and the manager is awaiting confirmation from the worker that it has done so. * `pending-controller-unpublish` The volume is successfully unpublished from the node, but has not yet been successfully unpublished on the controller.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum State {
    #[serde(rename = "pending-publish")]
    PendingPublish,
    #[serde(rename = "published")]
    Published,
    #[serde(rename = "pending-node-unpublish")]
    PendingNodeUnpublish,
    #[serde(rename = "pending-controller-unpublish")]
    PendingControllerUnpublish,
}
impl Default for State {
    fn default() -> State {
        Self::PendingPublish
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterVolumeSpec : Cluster-specific options used to create the volume.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumeSpec {
    /// Group defines the volume group of this volume. Volumes belonging to the same group can be referred to by group name when creating Services.  Referring to a volume by group instructs Swarm to treat volumes in that group interchangeably for the purpose of scheduling. Volumes with an empty string for a group technically all belong to the same, emptystring group.
    #[serde(rename = "Group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "AccessMode", skip_serializing_if = "Option::is_none")]
    pub access_mode: Option<Box<models::ClusterVolumeSpecAccessMode>>,
}
impl ClusterVolumeSpec {
    /// Cluster-specific options used to create the volume.
    pub fn new() -> ClusterVolumeSpec {
        ClusterVolumeSpec {
            group: None,
            access_mode: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterVolumeSpecAccessMode : Defines how the volume is used by tasks.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumeSpecAccessMode {
    /// The set of nodes this volume can be used on at one time. - `single` The volume may only be scheduled to one node at a time. - `multi` the volume may be scheduled to any supported number of nodes at a time.
    #[serde(rename = "Scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// The number and way that different tasks can use this volume at one time. - `none` The volume may only be used by one task at a time. - `readonly` The volume may be used by any number of tasks, but they all must mount the volume as readonly - `onewriter` The volume may be used by any number of tasks, but only one may mount it as read/write. - `all` The volume may have any number of readers and writers.
    #[serde(rename = "Sharing", skip_serializing_if = "Option::is_none")]
    pub sharing: Option<Sharing>,
    /// Options for using this volume as a Mount-type volume.      Either MountVolume or BlockVolume, but not both, must be     present.   properties:     FsType:       type: \"string\"       description: |         Specifies the filesystem type for the mount volume.         Optional.     MountFlags:       type: \"array\"       description: |         Flags to pass when mounting the volume. Optional.       items:         type: \"string\" BlockVolume:   type: \"object\"   description: |     Options for using this volume as a Block-type volume.     Intentionally empty.
    #[serde(rename = "MountVolume", skip_serializing_if = "Option::is_none")]
    pub mount_volume: Option<serde_json::Value>,
    /// Swarm Secrets that are passed to the CSI storage plugin when operating on this volume.
    #[serde(rename = "Secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<models::ClusterVolumeSpecAccessModeSecretsInner>>,
    #[serde(
        rename = "AccessibilityRequirements",
        skip_serializing_if = "Option::is_none"
    )]
    pub accessibility_requirements: Option<
        Box<models::ClusterVolumeSpecAccessModeAccessibilityRequirements>,
    >,
    #[serde(rename = "CapacityRange", skip_serializing_if = "Option::is_none")]
    pub capacity_range: Option<Box<models::ClusterVolumeSpecAccessModeCapacityRange>>,
    /// The availability of the volume for use in tasks. - `active` The volume is fully available for scheduling on the cluster - `pause` No new workloads should use the volume, but existing workloads are not stopped. - `drain` All workloads using this volume should be stopped and rescheduled, and no new ones should be started.
    #[serde(rename = "Availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
}
impl ClusterVolumeSpecAccessMode {
    /// Defines how the volume is used by tasks.
    pub fn new() -> ClusterVolumeSpecAccessMode {
        ClusterVolumeSpecAccessMode {
            scope: None,
            sharing: None,
            mount_volume: None,
            secrets: None,
            accessibility_requirements: None,
            capacity_range: None,
            availability: None,
        }
    }
}
/// The set of nodes this volume can be used on at one time. - `single` The volume may only be scheduled to one node at a time. - `multi` the volume may be scheduled to any supported number of nodes at a time.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Scope {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "multi")]
    Multi,
}
impl Default for Scope {
    fn default() -> Scope {
        Self::Single
    }
}
/// The number and way that different tasks can use this volume at one time. - `none` The volume may only be used by one task at a time. - `readonly` The volume may be used by any number of tasks, but they all must mount the volume as readonly - `onewriter` The volume may be used by any number of tasks, but only one may mount it as read/write. - `all` The volume may have any number of readers and writers.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Sharing {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "onewriter")]
    Onewriter,
    #[serde(rename = "all")]
    All,
}
impl Default for Sharing {
    fn default() -> Sharing {
        Self::None
    }
}
/// The availability of the volume for use in tasks. - `active` The volume is fully available for scheduling on the cluster - `pause` No new workloads should use the volume, but existing workloads are not stopped. - `drain` All workloads using this volume should be stopped and rescheduled, and no new ones should be started.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Availability {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "drain")]
    Drain,
}
impl Default for Availability {
    fn default() -> Availability {
        Self::Active
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterVolumeSpecAccessModeAccessibilityRequirements : Requirements for the accessible topology of the volume. These fields are optional. For an in-depth description of what these fields mean, see the CSI specification.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumeSpecAccessModeAccessibilityRequirements {
    /// A list of required topologies, at least one of which the volume must be accessible from.
    #[serde(rename = "Requisite", skip_serializing_if = "Option::is_none")]
    pub requisite: Option<Vec<std::collections::HashMap<String, String>>>,
    /// A list of topologies that the volume should attempt to be provisioned in.
    #[serde(rename = "Preferred", skip_serializing_if = "Option::is_none")]
    pub preferred: Option<Vec<std::collections::HashMap<String, String>>>,
}
impl ClusterVolumeSpecAccessModeAccessibilityRequirements {
    /// Requirements for the accessible topology of the volume. These fields are optional. For an in-depth description of what these fields mean, see the CSI specification.
    pub fn new() -> ClusterVolumeSpecAccessModeAccessibilityRequirements {
        ClusterVolumeSpecAccessModeAccessibilityRequirements {
            requisite: None,
            preferred: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterVolumeSpecAccessModeCapacityRange : The desired capacity that the volume should be created with. If empty, the plugin will decide the capacity.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumeSpecAccessModeCapacityRange {
    /// The volume must be at least this big. The value of 0 indicates an unspecified minimum
    #[serde(rename = "RequiredBytes", skip_serializing_if = "Option::is_none")]
    pub required_bytes: Option<i64>,
    /// The volume must not be bigger than this. The value of 0 indicates an unspecified maximum.
    #[serde(rename = "LimitBytes", skip_serializing_if = "Option::is_none")]
    pub limit_bytes: Option<i64>,
}
impl ClusterVolumeSpecAccessModeCapacityRange {
    /// The desired capacity that the volume should be created with. If empty, the plugin will decide the capacity.
    pub fn new() -> ClusterVolumeSpecAccessModeCapacityRange {
        ClusterVolumeSpecAccessModeCapacityRange {
            required_bytes: None,
            limit_bytes: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ClusterVolumeSpecAccessModeSecretsInner : One cluster volume secret entry. Defines a key-value pair that is passed to the plugin.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterVolumeSpecAccessModeSecretsInner {
    /// Key is the name of the key of the key-value pair passed to the plugin.
    #[serde(rename = "Key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Secret is the swarm Secret object from which to read data. This can be a Secret name or ID. The Secret data is retrieved by swarm and used as the value of the key-value pair passed to the plugin.
    #[serde(rename = "Secret", skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
impl ClusterVolumeSpecAccessModeSecretsInner {
    /// One cluster volume secret entry. Defines a key-value pair that is passed to the plugin.
    pub fn new() -> ClusterVolumeSpecAccessModeSecretsInner {
        ClusterVolumeSpecAccessModeSecretsInner {
            key: None,
            secret: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Commit : Commit holds the Git-commit (SHA1) that a binary was built from, as reported in the version-string of external tools, such as `containerd`, or `runC`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    /// Actual commit ID of external tool.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Commit ID of external tool expected by dockerd as set at build time.
    #[serde(rename = "Expected", skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
}
impl Commit {
    /// Commit holds the Git-commit (SHA1) that a binary was built from, as reported in the version-string of external tools, such as `containerd`, or `runC`.
    pub fn new() -> Commit {
        Commit { id: None, expected: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::ConfigSpec>>,
}
impl Config {
    pub fn new() -> Config {
        Config {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigCreateRequest {
    /// User-defined name of the config.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) config data.
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Templating", skip_serializing_if = "Option::is_none")]
    pub templating: Option<Box<models::Driver>>,
}
impl ConfigCreateRequest {
    pub fn new() -> ConfigCreateRequest {
        ConfigCreateRequest {
            name: None,
            labels: None,
            data: None,
            templating: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ConfigReference : The config-only network source to provide the configuration for this network.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigReference {
    /// The name of the config-only network that provides the network's configuration. The specified network must be an existing config-only network. Only network names are allowed, not network IDs.
    #[serde(rename = "Network", skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
}
impl ConfigReference {
    /// The config-only network source to provide the configuration for this network.
    pub fn new() -> ConfigReference {
        ConfigReference { network: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigSpec {
    /// User-defined name of the config.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) config data.
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Templating", skip_serializing_if = "Option::is_none")]
    pub templating: Option<Box<models::Driver>>,
}
impl ConfigSpec {
    pub fn new() -> ConfigSpec {
        ConfigSpec {
            name: None,
            labels: None,
            data: None,
            templating: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerConfig : Configuration for a container that is portable between hosts.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerConfig {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name to use for the container.
    #[serde(rename = "Domainname", skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// The user that commands are run as inside the container.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Whether to attach to `stdin`.
    #[serde(rename = "AttachStdin", skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Whether to attach to `stdout`.
    #[serde(rename = "AttachStdout", skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Whether to attach to `stderr`.
    #[serde(rename = "AttachStderr", skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
    #[serde(
        rename = "ExposedPorts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_ports: Option<
        Option<std::collections::HashMap<String, serde_json::Value>>,
    >,
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Close `stdin` after one attached client disconnects
    #[serde(rename = "StdinOnce", skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run specified as a string or an array of strings.
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Box<models::HealthConfig>>,
    /// Command is already escaped (Windows only)
    #[serde(
        rename = "ArgsEscaped",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub args_escaped: Option<Option<bool>>,
    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// An object mapping mount point paths inside the container to empty objects.
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The working directory for commands to run in.
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[serde(rename = "Entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Disable networking for the container.
    #[serde(
        rename = "NetworkDisabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_disabled: Option<Option<bool>>,
    /// MAC address of the container.  Deprecated: this field is deprecated in API v1.44 and up. Use EndpointSettings.MacAddress instead.
    #[serde(
        rename = "MacAddress",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<Option<String>>,
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(
        rename = "OnBuild",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_build: Option<Option<Vec<String>>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Signal to stop a container as a string or unsigned integer.
    #[serde(
        rename = "StopSignal",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_signal: Option<Option<String>>,
    /// Timeout to stop a container in seconds.
    #[serde(
        rename = "StopTimeout",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_timeout: Option<Option<i32>>,
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(
        rename = "Shell",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub shell: Option<Option<Vec<String>>>,
}
impl ContainerConfig {
    /// Configuration for a container that is portable between hosts.
    pub fn new() -> ContainerConfig {
        ContainerConfig {
            hostname: None,
            domainname: None,
            user: None,
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            exposed_ports: None,
            tty: None,
            open_stdin: None,
            stdin_once: None,
            env: None,
            cmd: None,
            healthcheck: None,
            args_escaped: None,
            image: None,
            volumes: None,
            working_dir: None,
            entrypoint: None,
            network_disabled: None,
            mac_address: None,
            on_build: None,
            labels: None,
            stop_signal: None,
            stop_timeout: None,
            shell: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerCreateRequest {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name to use for the container.
    #[serde(rename = "Domainname", skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// The user that commands are run as inside the container.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Whether to attach to `stdin`.
    #[serde(rename = "AttachStdin", skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Whether to attach to `stdout`.
    #[serde(rename = "AttachStdout", skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Whether to attach to `stderr`.
    #[serde(rename = "AttachStderr", skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
    #[serde(
        rename = "ExposedPorts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_ports: Option<
        Option<std::collections::HashMap<String, serde_json::Value>>,
    >,
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.
    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Close `stdin` after one attached client disconnects
    #[serde(rename = "StdinOnce", skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run specified as a string or an array of strings.
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Box<models::HealthConfig>>,
    /// Command is already escaped (Windows only)
    #[serde(
        rename = "ArgsEscaped",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub args_escaped: Option<Option<bool>>,
    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// An object mapping mount point paths inside the container to empty objects.
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The working directory for commands to run in.
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[serde(rename = "Entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Disable networking for the container.
    #[serde(
        rename = "NetworkDisabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_disabled: Option<Option<bool>>,
    /// MAC address of the container.  Deprecated: this field is deprecated in API v1.44 and up. Use EndpointSettings.MacAddress instead.
    #[serde(
        rename = "MacAddress",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<Option<String>>,
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(
        rename = "OnBuild",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_build: Option<Option<Vec<String>>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Signal to stop a container as a string or unsigned integer.
    #[serde(
        rename = "StopSignal",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_signal: Option<Option<String>>,
    /// Timeout to stop a container in seconds.
    #[serde(
        rename = "StopTimeout",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_timeout: Option<Option<i32>>,
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(
        rename = "Shell",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub shell: Option<Option<Vec<String>>>,
    #[serde(rename = "HostConfig", skip_serializing_if = "Option::is_none")]
    pub host_config: Option<Box<models::HostConfig>>,
    #[serde(rename = "NetworkingConfig", skip_serializing_if = "Option::is_none")]
    pub networking_config: Option<Box<models::NetworkingConfig>>,
}
impl ContainerCreateRequest {
    pub fn new() -> ContainerCreateRequest {
        ContainerCreateRequest {
            hostname: None,
            domainname: None,
            user: None,
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            exposed_ports: None,
            tty: None,
            open_stdin: None,
            stdin_once: None,
            env: None,
            cmd: None,
            healthcheck: None,
            args_escaped: None,
            image: None,
            volumes: None,
            working_dir: None,
            entrypoint: None,
            network_disabled: None,
            mac_address: None,
            on_build: None,
            labels: None,
            stop_signal: None,
            stop_timeout: None,
            shell: None,
            host_config: None,
            networking_config: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerCreateResponse : OK response to ContainerCreate operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerCreateResponse {
    /// The ID of the created container
    #[serde(rename = "Id")]
    pub id: String,
    /// Warnings encountered when creating the container
    #[serde(rename = "Warnings")]
    pub warnings: Vec<String>,
}
impl ContainerCreateResponse {
    /// OK response to ContainerCreate operation
    pub fn new(id: String, warnings: Vec<String>) -> ContainerCreateResponse {
        ContainerCreateResponse {
            id,
            warnings,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerInspectResponse {
    /// The ID of the container
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The time the container was created
    #[serde(rename = "Created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The path to the command being run
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// The arguments to the command being run
    #[serde(rename = "Args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(
        rename = "State",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub state: Option<Option<Box<models::ContainerState>>>,
    /// The container's image ID
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "ResolvConfPath", skip_serializing_if = "Option::is_none")]
    pub resolv_conf_path: Option<String>,
    #[serde(rename = "HostnamePath", skip_serializing_if = "Option::is_none")]
    pub hostname_path: Option<String>,
    #[serde(rename = "HostsPath", skip_serializing_if = "Option::is_none")]
    pub hosts_path: Option<String>,
    #[serde(rename = "LogPath", skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RestartCount", skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(rename = "MountLabel", skip_serializing_if = "Option::is_none")]
    pub mount_label: Option<String>,
    #[serde(rename = "ProcessLabel", skip_serializing_if = "Option::is_none")]
    pub process_label: Option<String>,
    #[serde(rename = "AppArmorProfile", skip_serializing_if = "Option::is_none")]
    pub app_armor_profile: Option<String>,
    /// IDs of exec instances that are running in the container.
    #[serde(
        rename = "ExecIDs",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exec_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "HostConfig", skip_serializing_if = "Option::is_none")]
    pub host_config: Option<Box<models::HostConfig>>,
    #[serde(rename = "GraphDriver", skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<Box<models::DriverData>>,
    /// The size of files that have been created or changed by this container.
    #[serde(rename = "SizeRw", skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    /// The total size of all the files in this container.
    #[serde(rename = "SizeRootFs", skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    #[serde(rename = "Mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::MountPoint>>,
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::ContainerConfig>>,
    #[serde(rename = "NetworkSettings", skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<Box<models::NetworkSettings>>,
}
impl ContainerInspectResponse {
    pub fn new() -> ContainerInspectResponse {
        ContainerInspectResponse {
            id: None,
            created: None,
            path: None,
            args: None,
            state: None,
            image: None,
            resolv_conf_path: None,
            hostname_path: None,
            hosts_path: None,
            log_path: None,
            name: None,
            restart_count: None,
            driver: None,
            platform: None,
            mount_label: None,
            process_label: None,
            app_armor_profile: None,
            exec_ids: None,
            host_config: None,
            graph_driver: None,
            size_rw: None,
            size_root_fs: None,
            mounts: None,
            config: None,
            network_settings: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerPruneResponse {
    /// Container IDs that were deleted
    #[serde(rename = "ContainersDeleted", skip_serializing_if = "Option::is_none")]
    pub containers_deleted: Option<Vec<String>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed", skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
impl ContainerPruneResponse {
    pub fn new() -> ContainerPruneResponse {
        ContainerPruneResponse {
            containers_deleted: None,
            space_reclaimed: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerState : ContainerState stores container's running state. It's part of ContainerJSONBase and will be returned by the \"inspect\" command.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerState {
    /// String representation of the container state. Can be one of \"created\", \"running\", \"paused\", \"restarting\", \"removing\", \"exited\", or \"dead\".
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// Whether this container is running.  Note that a running container can be _paused_. The `Running` and `Paused` booleans are not mutually exclusive:  When pausing a container (on Linux), the freezer cgroup is used to suspend all processes in the container. Freezing the process requires the process to be running. As a result, paused containers are both `Running` _and_ `Paused`.  Use the `Status` field instead to determine if a container's state is \"running\".
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    /// Whether this container is paused.
    #[serde(rename = "Paused", skip_serializing_if = "Option::is_none")]
    pub paused: Option<bool>,
    /// Whether this container is restarting.
    #[serde(rename = "Restarting", skip_serializing_if = "Option::is_none")]
    pub restarting: Option<bool>,
    /// Whether a process within this container has been killed because it ran out of memory since the container was last started.
    #[serde(rename = "OOMKilled", skip_serializing_if = "Option::is_none")]
    pub oom_killed: Option<bool>,
    #[serde(rename = "Dead", skip_serializing_if = "Option::is_none")]
    pub dead: Option<bool>,
    /// The process ID of this container
    #[serde(rename = "Pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    /// The last exit code of this container
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The time when this container was last started.
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// The time when this container last exited.
    #[serde(rename = "FinishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(
        rename = "Health",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub health: Option<Option<Box<models::Health>>>,
}
impl ContainerState {
    /// ContainerState stores container's running state. It's part of ContainerJSONBase and will be returned by the \"inspect\" command.
    pub fn new() -> ContainerState {
        ContainerState {
            status: None,
            running: None,
            paused: None,
            restarting: None,
            oom_killed: None,
            dead: None,
            pid: None,
            exit_code: None,
            error: None,
            started_at: None,
            finished_at: None,
            health: None,
        }
    }
}
/// String representation of the container state. Can be one of \"created\", \"running\", \"paused\", \"restarting\", \"removing\", \"exited\", or \"dead\".
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Status {
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "restarting")]
    Restarting,
    #[serde(rename = "removing")]
    Removing,
    #[serde(rename = "exited")]
    Exited,
    #[serde(rename = "dead")]
    Dead,
}
impl Default for Status {
    fn default() -> Status {
        Self::Created
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerStatus : represents the status of a container.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerStatus {
    #[serde(rename = "ContainerID", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "PID", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
}
impl ContainerStatus {
    /// represents the status of a container.
    pub fn new() -> ContainerStatus {
        ContainerStatus {
            container_id: None,
            pid: None,
            exit_code: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummary {
    /// The ID of this container
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The names that this container has been given
    #[serde(rename = "Names", skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    /// The name of the image used when creating this container
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// The ID of the image that this container was created from
    #[serde(rename = "ImageID", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    /// Command to run when starting the container
    #[serde(rename = "Command", skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// When the container was created
    #[serde(rename = "Created", skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    /// The ports exposed by this container
    #[serde(rename = "Ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<models::Port>>,
    /// The size of files that have been created or changed by this container
    #[serde(rename = "SizeRw", skip_serializing_if = "Option::is_none")]
    pub size_rw: Option<i64>,
    /// The total size of all the files in this container
    #[serde(rename = "SizeRootFs", skip_serializing_if = "Option::is_none")]
    pub size_root_fs: Option<i64>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// The state of this container (e.g. `Exited`)
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Additional human-readable status of this container (e.g. `Exit 0`)
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "HostConfig", skip_serializing_if = "Option::is_none")]
    pub host_config: Option<Box<models::ContainerSummaryHostConfig>>,
    #[serde(rename = "NetworkSettings", skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<Box<models::ContainerSummaryNetworkSettings>>,
    #[serde(rename = "Mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::MountPoint>>,
}
impl ContainerSummary {
    pub fn new() -> ContainerSummary {
        ContainerSummary {
            id: None,
            names: None,
            image: None,
            image_id: None,
            command: None,
            created: None,
            ports: None,
            size_rw: None,
            size_root_fs: None,
            labels: None,
            state: None,
            status: None,
            host_config: None,
            network_settings: None,
            mounts: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummaryHostConfig {
    #[serde(rename = "NetworkMode", skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// Arbitrary key-value metadata attached to container
    #[serde(
        rename = "Annotations",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub annotations: Option<Option<std::collections::HashMap<String, String>>>,
}
impl ContainerSummaryHostConfig {
    pub fn new() -> ContainerSummaryHostConfig {
        ContainerSummaryHostConfig {
            network_mode: None,
            annotations: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerSummaryNetworkSettings : A summary of the container's network settings
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerSummaryNetworkSettings {
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<std::collections::HashMap<String, models::EndpointSettings>>,
}
impl ContainerSummaryNetworkSettings {
    /// A summary of the container's network settings
    pub fn new() -> ContainerSummaryNetworkSettings {
        ContainerSummaryNetworkSettings {
            networks: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerTopResponse : OK response to ContainerTop operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerTopResponse {
    /// The ps column titles
    #[serde(rename = "Titles", skip_serializing_if = "Option::is_none")]
    pub titles: Option<Vec<String>>,
    /// Each process running in the container, where each is process is an array of values corresponding to the titles.
    #[serde(rename = "Processes", skip_serializing_if = "Option::is_none")]
    pub processes: Option<Vec<Vec<String>>>,
}
impl ContainerTopResponse {
    /// OK response to ContainerTop operation
    pub fn new() -> ContainerTopResponse {
        ContainerTopResponse {
            titles: None,
            processes: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerUpdateRequest {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[serde(rename = "CpuShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i32>,
    /// Memory limit in bytes.
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.
    #[serde(rename = "CgroupParent", skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Block IO weight (relative weight).
    #[serde(rename = "BlkioWeight", skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u32>,
    /// Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ```
    #[serde(rename = "BlkioWeightDevice", skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<models::ResourcesBlkioWeightDeviceInner>>,
    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<models::ThrottleDevice>>,
    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<models::ThrottleDevice>>,
    /// Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<models::ThrottleDevice>>,
    /// Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<models::ThrottleDevice>>,
    /// The length of a CPU period in microseconds.
    #[serde(rename = "CpuPeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// Microseconds of CPU time that the container can get in a CPU period.
    #[serde(rename = "CpuQuota", skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimePeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimeRuntime", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[serde(rename = "CpusetCpus", skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[serde(rename = "CpusetMems", skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// A list of devices to add to the container.
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<models::DeviceMapping>>,
    /// a list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules", skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    /// A list of requests for devices to be sent to device drivers.
    #[serde(rename = "DeviceRequests", skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<models::DeviceRequest>>,
    /// Hard limit for kernel TCP buffer memory (in bytes). Depending on the OCI runtime in use, this option may be ignored. It is no longer supported by the default (runc) runtime.  This field is omitted when empty.
    #[serde(rename = "KernelMemoryTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    /// Memory soft limit in bytes.
    #[serde(rename = "MemoryReservation", skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(rename = "MemorySwap", skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(rename = "MemorySwappiness", skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<u64>,
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[serde(rename = "NanoCpus", skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// Disable OOM Killer for the container.
    #[serde(rename = "OomKillDisable", skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(
        rename = "Init",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub init: Option<Option<bool>>,
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.
    #[serde(
        rename = "PidsLimit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pids_limit: Option<Option<i64>>,
    /// A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ```
    #[serde(rename = "Ulimits", skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<models::ResourcesUlimitsInner>>,
    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuCount", skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuPercent", skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps", skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[serde(rename = "IOMaximumBandwidth", skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
    #[serde(rename = "RestartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<Box<models::RestartPolicy>>,
}
impl ContainerUpdateRequest {
    pub fn new() -> ContainerUpdateRequest {
        ContainerUpdateRequest {
            cpu_shares: None,
            memory: None,
            cgroup_parent: None,
            blkio_weight: None,
            blkio_weight_device: None,
            blkio_device_read_bps: None,
            blkio_device_write_bps: None,
            blkio_device_read_i_ops: None,
            blkio_device_write_i_ops: None,
            cpu_period: None,
            cpu_quota: None,
            cpu_realtime_period: None,
            cpu_realtime_runtime: None,
            cpuset_cpus: None,
            cpuset_mems: None,
            devices: None,
            device_cgroup_rules: None,
            device_requests: None,
            kernel_memory_tcp: None,
            memory_reservation: None,
            memory_swap: None,
            memory_swappiness: None,
            nano_cpus: None,
            oom_kill_disable: None,
            init: None,
            pids_limit: None,
            ulimits: None,
            cpu_count: None,
            cpu_percent: None,
            io_maximum_i_ops: None,
            io_maximum_bandwidth: None,
            restart_policy: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerUpdateResponse : OK response to ContainerUpdate operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerUpdateResponse {
    #[serde(rename = "Warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
impl ContainerUpdateResponse {
    /// OK response to ContainerUpdate operation
    pub fn new() -> ContainerUpdateResponse {
        ContainerUpdateResponse {
            warnings: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerWaitExitError : container waiting error, if any
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerWaitExitError {
    /// Details of an error
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ContainerWaitExitError {
    /// container waiting error, if any
    pub fn new() -> ContainerWaitExitError {
        ContainerWaitExitError {
            message: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerWaitResponse : OK response to ContainerWait operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerWaitResponse {
    /// Exit code of the container
    #[serde(rename = "StatusCode")]
    pub status_code: i64,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ContainerWaitExitError>>,
}
impl ContainerWaitResponse {
    /// OK response to ContainerWait operation
    pub fn new(status_code: i64) -> ContainerWaitResponse {
        ContainerWaitResponse {
            status_code,
            error: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerdInfo : Information for connecting to the containerd instance that is used by the daemon. This is included for debugging purposes only.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerdInfo {
    /// The address of the containerd socket.
    #[serde(rename = "Address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "Namespaces", skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Box<models::ContainerdInfoNamespaces>>,
}
impl ContainerdInfo {
    /// Information for connecting to the containerd instance that is used by the daemon. This is included for debugging purposes only.
    pub fn new() -> ContainerdInfo {
        ContainerdInfo {
            address: None,
            namespaces: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ContainerdInfoNamespaces : The namespaces that the daemon uses for running containers and plugins in containerd. These namespaces can be configured in the daemon configuration, and are considered to be used exclusively by the daemon, Tampering with the containerd instance may cause unexpected behavior.  As these namespaces are considered to be exclusively accessed by the daemon, it is not recommended to change these values, or to change them to a value that is used by other systems, such as cri-containerd.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContainerdInfoNamespaces {
    /// The default containerd namespace used for containers managed by the daemon.  The default namespace for containers is \"moby\", but will be suffixed with the `<uid>.<gid>` of the remapped `root` if user-namespaces are enabled and the containerd image-store is used.
    #[serde(rename = "Containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<String>,
    /// The default containerd namespace used for plugins managed by the daemon.  The default namespace for plugins is \"plugins.moby\", but will be suffixed with the `<uid>.<gid>` of the remapped `root` if user-namespaces are enabled and the containerd image-store is used.
    #[serde(rename = "Plugins", skip_serializing_if = "Option::is_none")]
    pub plugins: Option<String>,
}
impl ContainerdInfoNamespaces {
    /// The namespaces that the daemon uses for running containers and plugins in containerd. These namespaces can be configured in the daemon configuration, and are considered to be used exclusively by the daemon, Tampering with the containerd instance may cause unexpected behavior.  As these namespaces are considered to be exclusively accessed by the daemon, it is not recommended to change these values, or to change them to a value that is used by other systems, such as cri-containerd.
    pub fn new() -> ContainerdInfoNamespaces {
        ContainerdInfoNamespaces {
            containers: None,
            plugins: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateImageInfo {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "errorDetail", skip_serializing_if = "Option::is_none")]
    pub error_detail: Option<Box<models::ErrorDetail>>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail", skip_serializing_if = "Option::is_none")]
    pub progress_detail: Option<Box<models::ProgressDetail>>,
}
impl CreateImageInfo {
    pub fn new() -> CreateImageInfo {
        CreateImageInfo {
            id: None,
            error: None,
            error_detail: None,
            status: None,
            progress: None,
            progress_detail: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// DeviceMapping : A device mapping between the host and container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceMapping {
    #[serde(rename = "PathOnHost", skip_serializing_if = "Option::is_none")]
    pub path_on_host: Option<String>,
    #[serde(rename = "PathInContainer", skip_serializing_if = "Option::is_none")]
    pub path_in_container: Option<String>,
    #[serde(rename = "CgroupPermissions", skip_serializing_if = "Option::is_none")]
    pub cgroup_permissions: Option<String>,
}
impl DeviceMapping {
    /// A device mapping between the host and container
    pub fn new() -> DeviceMapping {
        DeviceMapping {
            path_on_host: None,
            path_in_container: None,
            cgroup_permissions: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// DeviceRequest : A request for devices to be sent to device drivers
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeviceRequest {
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    #[serde(rename = "Count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "DeviceIDs", skip_serializing_if = "Option::is_none")]
    pub device_ids: Option<Vec<String>>,
    /// A list of capabilities; an OR list of AND lists of capabilities.
    #[serde(rename = "Capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Vec<String>>>,
    /// Driver-specific options, specified as a key/value pairs. These options are passed directly to the driver.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
}
impl DeviceRequest {
    /// A request for devices to be sent to device drivers
    pub fn new() -> DeviceRequest {
        DeviceRequest {
            driver: None,
            count: None,
            device_ids: None,
            capabilities: None,
            options: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// DistributionInspect : Describes the result obtained from contacting the registry to retrieve image metadata.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DistributionInspect {
    #[serde(rename = "Descriptor")]
    pub descriptor: Box<models::OciDescriptor>,
    /// An array containing all platforms supported by the image.
    #[serde(rename = "Platforms")]
    pub platforms: Vec<models::OciPlatform>,
}
impl DistributionInspect {
    /// Describes the result obtained from contacting the registry to retrieve image metadata.
    pub fn new(
        descriptor: models::OciDescriptor,
        platforms: Vec<models::OciPlatform>,
    ) -> DistributionInspect {
        DistributionInspect {
            descriptor: Box::new(descriptor),
            platforms,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Driver : Driver represents a driver (network, logging, secrets).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Driver {
    /// Name of the driver.
    #[serde(rename = "Name")]
    pub name: String,
    /// Key/value map of driver-specific options.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
}
impl Driver {
    /// Driver represents a driver (network, logging, secrets).
    pub fn new(name: String) -> Driver {
        Driver { name, options: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// DriverData : Information about the storage driver used to store the container's and image's filesystem.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DriverData {
    /// Name of the storage driver.
    #[serde(rename = "Name")]
    pub name: String,
    /// Low-level storage metadata, provided as key/value pairs.  This information is driver-specific, and depends on the storage-driver in use, and should be used for informational purposes only.
    #[serde(rename = "Data")]
    pub data: std::collections::HashMap<String, String>,
}
impl DriverData {
    /// Information about the storage driver used to store the container's and image's filesystem.
    pub fn new(
        name: String,
        data: std::collections::HashMap<String, String>,
    ) -> DriverData {
        DriverData { name, data }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// EndpointIpamConfig : EndpointIPAMConfig represents an endpoint's IPAM configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointIpamConfig {
    #[serde(rename = "IPv4Address", skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "IPv6Address", skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "LinkLocalIPs", skip_serializing_if = "Option::is_none")]
    pub link_local_ips: Option<Vec<String>>,
}
impl EndpointIpamConfig {
    /// EndpointIPAMConfig represents an endpoint's IPAM configuration.
    pub fn new() -> EndpointIpamConfig {
        EndpointIpamConfig {
            ipv4_address: None,
            ipv6_address: None,
            link_local_ips: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointPortConfig {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// The port inside the container.
    #[serde(rename = "TargetPort", skip_serializing_if = "Option::is_none")]
    pub target_port: Option<i32>,
    /// The port on the swarm hosts.
    #[serde(rename = "PublishedPort", skip_serializing_if = "Option::is_none")]
    pub published_port: Option<i32>,
    /// The mode in which port is published.  <p><br /></p>  - \"ingress\" makes the target port accessible on every node,   regardless of whether there is a task for the service running on   that node or not. - \"host\" bypasses the routing mesh and publish the port directly on   the swarm node where that service is running.
    #[serde(rename = "PublishMode", skip_serializing_if = "Option::is_none")]
    pub publish_mode: Option<PublishMode>,
}
impl EndpointPortConfig {
    pub fn new() -> EndpointPortConfig {
        EndpointPortConfig {
            name: None,
            protocol: None,
            target_port: None,
            published_port: None,
            publish_mode: None,
        }
    }
}
///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Protocol {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}
impl Default for Protocol {
    fn default() -> Protocol {
        Self::Tcp
    }
}
/// The mode in which port is published.  <p><br /></p>  - \"ingress\" makes the target port accessible on every node,   regardless of whether there is a task for the service running on   that node or not. - \"host\" bypasses the routing mesh and publish the port directly on   the swarm node where that service is running.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum PublishMode {
    #[serde(rename = "ingress")]
    Ingress,
    #[serde(rename = "host")]
    Host,
}
impl Default for PublishMode {
    fn default() -> PublishMode {
        Self::Ingress
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// EndpointSettings : Configuration for a network endpoint.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointSettings {
    #[serde(
        rename = "IPAMConfig",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ipam_config: Option<Option<Box<models::EndpointIpamConfig>>>,
    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// MAC address for the endpoint on this network. The network driver might ignore this parameter.
    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// DriverOpts is a mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[serde(
        rename = "DriverOpts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub driver_opts: Option<Option<std::collections::HashMap<String, String>>>,
    /// Unique ID of the network.
    #[serde(rename = "NetworkID", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// Unique ID for the service endpoint in a Sandbox.
    #[serde(rename = "EndpointID", skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// Gateway address for this network.
    #[serde(rename = "Gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// IPv4 address.
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Mask length of the IPv4 address.
    #[serde(rename = "IPPrefixLen", skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i32>,
    /// IPv6 gateway address.
    #[serde(rename = "IPv6Gateway", skip_serializing_if = "Option::is_none")]
    pub ipv6_gateway: Option<String>,
    /// Global IPv6 address.
    #[serde(rename = "GlobalIPv6Address", skip_serializing_if = "Option::is_none")]
    pub global_ipv6_address: Option<String>,
    /// Mask length of the global IPv6 address.
    #[serde(rename = "GlobalIPv6PrefixLen", skip_serializing_if = "Option::is_none")]
    pub global_ipv6_prefix_len: Option<i64>,
    /// List of all DNS names an endpoint has on a specific network. This list is based on the container name, network aliases, container short ID, and hostname.  These DNS names are non-fully qualified but can contain several dots. You can get fully qualified DNS names by appending `.<network-name>`. For instance, if container name is `my.ctr` and the network is named `testnet`, `DNSNames` will contain `my.ctr` and the FQDN will be `my.ctr.testnet`.
    #[serde(rename = "DNSNames", skip_serializing_if = "Option::is_none")]
    pub dns_names: Option<Vec<String>>,
}
impl EndpointSettings {
    /// Configuration for a network endpoint.
    pub fn new() -> EndpointSettings {
        EndpointSettings {
            ipam_config: None,
            links: None,
            mac_address: None,
            aliases: None,
            driver_opts: None,
            network_id: None,
            endpoint_id: None,
            gateway: None,
            ip_address: None,
            ip_prefix_len: None,
            ipv6_gateway: None,
            global_ipv6_address: None,
            global_ipv6_prefix_len: None,
            dns_names: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// EndpointSpec : Properties that can be configured to access and load balance a service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EndpointSpec {
    /// The mode of resolution to use for internal load balancing between tasks.
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// List of exposed ports that this service is accessible on from the outside. Ports can only be provided if `vip` resolution mode is used.
    #[serde(rename = "Ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<models::EndpointPortConfig>>,
}
impl EndpointSpec {
    /// Properties that can be configured to access and load balance a service.
    pub fn new() -> EndpointSpec {
        EndpointSpec {
            mode: None,
            ports: None,
        }
    }
}
/// The mode of resolution to use for internal load balancing between tasks.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Mode {
    #[serde(rename = "vip")]
    Vip,
    #[serde(rename = "dnsrr")]
    Dnsrr,
}
impl Default for Mode {
    fn default() -> Mode {
        Self::Vip
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// EngineDescription : EngineDescription provides information about an engine.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EngineDescription {
    #[serde(rename = "EngineVersion", skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Plugins", skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<models::EngineDescriptionPluginsInner>>,
}
impl EngineDescription {
    /// EngineDescription provides information about an engine.
    pub fn new() -> EngineDescription {
        EngineDescription {
            engine_version: None,
            labels: None,
            plugins: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EngineDescriptionPluginsInner {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
impl EngineDescriptionPluginsInner {
    pub fn new() -> EngineDescriptionPluginsInner {
        EngineDescriptionPluginsInner {
            r#type: None,
            name: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ErrorDetail {
    pub fn new() -> ErrorDetail {
        ErrorDetail {
            code: None,
            message: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ErrorResponse : Represents an error.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// The error message.
    #[serde(rename = "message")]
    pub message: String,
}
impl ErrorResponse {
    /// Represents an error.
    pub fn new(message: String) -> ErrorResponse {
        ErrorResponse { message }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// EventActor : Actor describes something that generates events, like a container, network, or a volume.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventActor {
    /// The ID of the object emitting the event
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Various key/value attributes of the object, depending on its type.
    #[serde(rename = "Attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::HashMap<String, String>>,
}
impl EventActor {
    /// Actor describes something that generates events, like a container, network, or a volume.
    pub fn new() -> EventActor {
        EventActor {
            id: None,
            attributes: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// EventMessage : EventMessage represents the information an event contains.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventMessage {
    /// The type of object emitting the event
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// The type of event
    #[serde(rename = "Action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Actor", skip_serializing_if = "Option::is_none")]
    pub actor: Option<Box<models::EventActor>>,
    /// Scope of the event. Engine events are `local` scope. Cluster (Swarm) events are `swarm` scope.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// Timestamp of event
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Timestamp of event, with nanosecond accuracy
    #[serde(rename = "timeNano", skip_serializing_if = "Option::is_none")]
    pub time_nano: Option<i64>,
}
impl EventMessage {
    /// EventMessage represents the information an event contains.
    pub fn new() -> EventMessage {
        EventMessage {
            r#type: None,
            action: None,
            actor: None,
            scope: None,
            time: None,
            time_nano: None,
        }
    }
}
/// The type of object emitting the event
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Type {
    #[serde(rename = "builder")]
    Builder,
    #[serde(rename = "config")]
    Config,
    #[serde(rename = "container")]
    Container,
    #[serde(rename = "daemon")]
    Daemon,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "network")]
    Network,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "plugin")]
    Plugin,
    #[serde(rename = "secret")]
    Secret,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "volume")]
    Volume,
}
impl Default for Type {
    fn default() -> Type {
        Self::Builder
    }
}
/// Scope of the event. Engine events are `local` scope. Cluster (Swarm) events are `swarm` scope.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Scope {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "swarm")]
    Swarm,
}
impl Default for Scope {
    fn default() -> Scope {
        Self::Local
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecConfig {
    /// Attach to `stdin` of the exec command.
    #[serde(rename = "AttachStdin", skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Attach to `stdout` of the exec command.
    #[serde(rename = "AttachStdout", skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Attach to `stderr` of the exec command.
    #[serde(rename = "AttachStderr", skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// Initial console size, as an `[height, width]` array.
    #[serde(
        rename = "ConsoleSize",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub console_size: Option<Option<Vec<u32>>>,
    /// Override the key sequence for detaching a container. Format is a single character `[a-Z]` or `ctrl-<value>` where `<value>` is one of: `a-z`, `@`, `^`, `[`, `,` or `_`.
    #[serde(rename = "DetachKeys", skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// A list of environment variables in the form `[\"VAR=value\", ...]`.
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run, as a string or array of strings.
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    /// Runs the exec process with extended privileges.
    #[serde(rename = "Privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// The user, and optionally, group to run the exec process inside the container. Format is one of: `user`, `user:group`, `uid`, or `uid:gid`.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the exec process inside the container.
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
}
impl ExecConfig {
    pub fn new() -> ExecConfig {
        ExecConfig {
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            console_size: None,
            detach_keys: None,
            tty: None,
            env: None,
            cmd: None,
            privileged: None,
            user: None,
            working_dir: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecInspectResponse {
    #[serde(rename = "CanRemove", skip_serializing_if = "Option::is_none")]
    pub can_remove: Option<bool>,
    #[serde(rename = "DetachKeys", skip_serializing_if = "Option::is_none")]
    pub detach_keys: Option<String>,
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Running", skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "ProcessConfig", skip_serializing_if = "Option::is_none")]
    pub process_config: Option<Box<models::ProcessConfig>>,
    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    #[serde(rename = "OpenStderr", skip_serializing_if = "Option::is_none")]
    pub open_stderr: Option<bool>,
    #[serde(rename = "OpenStdout", skip_serializing_if = "Option::is_none")]
    pub open_stdout: Option<bool>,
    #[serde(rename = "ContainerID", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// The system process ID for the exec process.
    #[serde(rename = "Pid", skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
}
impl ExecInspectResponse {
    pub fn new() -> ExecInspectResponse {
        ExecInspectResponse {
            can_remove: None,
            detach_keys: None,
            id: None,
            running: None,
            exit_code: None,
            process_config: None,
            open_stdin: None,
            open_stderr: None,
            open_stdout: None,
            container_id: None,
            pid: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecStartConfig {
    /// Detach from the command.
    #[serde(rename = "Detach", skip_serializing_if = "Option::is_none")]
    pub detach: Option<bool>,
    /// Allocate a pseudo-TTY.
    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Initial console size, as an `[height, width]` array.
    #[serde(
        rename = "ConsoleSize",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub console_size: Option<Option<Vec<u32>>>,
}
impl ExecStartConfig {
    pub fn new() -> ExecStartConfig {
        ExecStartConfig {
            detach: None,
            tty: None,
            console_size: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// FilesystemChange : Change in the container's filesystem.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FilesystemChange {
    /// Path to file or directory that has changed.
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Kind")]
    pub kind: models::ChangeType,
}
impl FilesystemChange {
    /// Change in the container's filesystem.
    pub fn new(path: String, kind: models::ChangeType) -> FilesystemChange {
        FilesystemChange { path, kind }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResourcesInner {
    #[serde(rename = "NamedResourceSpec", skip_serializing_if = "Option::is_none")]
    pub named_resource_spec: Option<Box<models::GenericResourcesInnerNamedResourceSpec>>,
    #[serde(rename = "DiscreteResourceSpec", skip_serializing_if = "Option::is_none")]
    pub discrete_resource_spec: Option<
        Box<models::GenericResourcesInnerDiscreteResourceSpec>,
    >,
}
impl GenericResourcesInner {
    pub fn new() -> GenericResourcesInner {
        GenericResourcesInner {
            named_resource_spec: None,
            discrete_resource_spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResourcesInnerDiscreteResourceSpec {
    #[serde(rename = "Kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
impl GenericResourcesInnerDiscreteResourceSpec {
    pub fn new() -> GenericResourcesInnerDiscreteResourceSpec {
        GenericResourcesInnerDiscreteResourceSpec {
            kind: None,
            value: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericResourcesInnerNamedResourceSpec {
    #[serde(rename = "Kind", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl GenericResourcesInnerNamedResourceSpec {
    pub fn new() -> GenericResourcesInnerNamedResourceSpec {
        GenericResourcesInnerNamedResourceSpec {
            kind: None,
            value: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Health : Health stores information about the container's healthcheck results.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Health {
    /// Status is one of `none`, `starting`, `healthy` or `unhealthy`  - \"none\"      Indicates there is no healthcheck - \"starting\"  Starting indicates that the container is not yet ready - \"healthy\"   Healthy indicates that the container is running correctly - \"unhealthy\" Unhealthy indicates that the container has a problem
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// FailingStreak is the number of consecutive failures
    #[serde(rename = "FailingStreak", skip_serializing_if = "Option::is_none")]
    pub failing_streak: Option<i32>,
    /// Log contains the last few results (oldest first)
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<models::HealthcheckResult>>,
}
impl Health {
    /// Health stores information about the container's healthcheck results.
    pub fn new() -> Health {
        Health {
            status: None,
            failing_streak: None,
            log: None,
        }
    }
}
/// Status is one of `none`, `starting`, `healthy` or `unhealthy`  - \"none\"      Indicates there is no healthcheck - \"starting\"  Starting indicates that the container is not yet ready - \"healthy\"   Healthy indicates that the container is running correctly - \"unhealthy\" Unhealthy indicates that the container has a problem
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Status {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "healthy")]
    Healthy,
    #[serde(rename = "unhealthy")]
    Unhealthy,
}
impl Default for Status {
    fn default() -> Status {
        Self::None
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// HealthConfig : A test to perform to check that the container is healthy.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthConfig {
    /// The test to perform. Possible values are:  - `[]` inherit healthcheck from image or parent image - `[\"NONE\"]` disable healthcheck - `[\"CMD\", args...]` exec arguments directly - `[\"CMD-SHELL\", command]` run command with system's default shell
    #[serde(rename = "Test", skip_serializing_if = "Option::is_none")]
    pub test: Option<Vec<String>>,
    /// The time to wait between checks in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[serde(rename = "Interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// The time to wait before considering the check to have hung. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[serde(rename = "Timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// The number of consecutive failures needed to consider a container as unhealthy. 0 means inherit.
    #[serde(rename = "Retries", skip_serializing_if = "Option::is_none")]
    pub retries: Option<i32>,
    /// Start period for the container to initialize before starting health-retries countdown in nanoseconds. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[serde(rename = "StartPeriod", skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,
    /// The time to wait between checks in nanoseconds during the start period. It should be 0 or at least 1000000 (1 ms). 0 means inherit.
    #[serde(rename = "StartInterval", skip_serializing_if = "Option::is_none")]
    pub start_interval: Option<i64>,
}
impl HealthConfig {
    /// A test to perform to check that the container is healthy.
    pub fn new() -> HealthConfig {
        HealthConfig {
            test: None,
            interval: None,
            timeout: None,
            retries: None,
            start_period: None,
            start_interval: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// HealthcheckResult : HealthcheckResult stores information about a single run of a healthcheck probe
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HealthcheckResult {
    /// Date and time at which this check started in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// Date and time at which this check ended in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "End", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// ExitCode meanings:  - `0` healthy - `1` unhealthy - `2` reserved (considered unhealthy) - other values: error running probe
    #[serde(rename = "ExitCode", skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    /// Output from last check
    #[serde(rename = "Output", skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}
impl HealthcheckResult {
    /// HealthcheckResult stores information about a single run of a healthcheck probe
    pub fn new() -> HealthcheckResult {
        HealthcheckResult {
            start: None,
            end: None,
            exit_code: None,
            output: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// HistoryResponseItem : individual image layer information in response to ImageHistory operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HistoryResponseItem {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Created")]
    pub created: i64,
    #[serde(rename = "CreatedBy")]
    pub created_by: String,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
    #[serde(rename = "Size")]
    pub size: i64,
    #[serde(rename = "Comment")]
    pub comment: String,
}
impl HistoryResponseItem {
    /// individual image layer information in response to ImageHistory operation
    pub fn new(
        id: String,
        created: i64,
        created_by: String,
        tags: Vec<String>,
        size: i64,
        comment: String,
    ) -> HistoryResponseItem {
        HistoryResponseItem {
            id,
            created,
            created_by,
            tags,
            size,
            comment,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// HostConfig : Container configuration that depends on the host we are running on
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostConfig {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[serde(rename = "CpuShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i32>,
    /// Memory limit in bytes.
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.
    #[serde(rename = "CgroupParent", skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Block IO weight (relative weight).
    #[serde(rename = "BlkioWeight", skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u32>,
    /// Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ```
    #[serde(rename = "BlkioWeightDevice", skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<models::ResourcesBlkioWeightDeviceInner>>,
    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<models::ThrottleDevice>>,
    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<models::ThrottleDevice>>,
    /// Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<models::ThrottleDevice>>,
    /// Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<models::ThrottleDevice>>,
    /// The length of a CPU period in microseconds.
    #[serde(rename = "CpuPeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// Microseconds of CPU time that the container can get in a CPU period.
    #[serde(rename = "CpuQuota", skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimePeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimeRuntime", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[serde(rename = "CpusetCpus", skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[serde(rename = "CpusetMems", skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// A list of devices to add to the container.
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<models::DeviceMapping>>,
    /// a list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules", skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    /// A list of requests for devices to be sent to device drivers.
    #[serde(rename = "DeviceRequests", skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<models::DeviceRequest>>,
    /// Hard limit for kernel TCP buffer memory (in bytes). Depending on the OCI runtime in use, this option may be ignored. It is no longer supported by the default (runc) runtime.  This field is omitted when empty.
    #[serde(rename = "KernelMemoryTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    /// Memory soft limit in bytes.
    #[serde(rename = "MemoryReservation", skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(rename = "MemorySwap", skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(rename = "MemorySwappiness", skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<u64>,
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[serde(rename = "NanoCpus", skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// Disable OOM Killer for the container.
    #[serde(rename = "OomKillDisable", skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(
        rename = "Init",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub init: Option<Option<bool>>,
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.
    #[serde(
        rename = "PidsLimit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pids_limit: Option<Option<i64>>,
    /// A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ```
    #[serde(rename = "Ulimits", skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<models::ResourcesUlimitsInner>>,
    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuCount", skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuPercent", skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps", skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[serde(rename = "IOMaximumBandwidth", skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
    /// A list of volume bindings for this container. Each volume binding is a string in one of these forms:  - `host-src:container-dest[:options]` to bind-mount a host path   into the container. Both `host-src`, and `container-dest` must   be an _absolute_ path. - `volume-name:container-dest[:options]` to bind-mount a volume   managed by a volume driver into the container. `container-dest`   must be an _absolute_ path.  `options` is an optional, comma-delimited list of:  - `nocopy` disables automatic copying of data from the container   path to the volume. The `nocopy` flag only applies to named volumes. - `[ro|rw]` mounts a volume read-only or read-write, respectively.   If omitted or set to `rw`, volumes are mounted read-write. - `[z|Z]` applies SELinux labels to allow or deny multiple containers   to read and write to the same volume.     - `z`: a _shared_ content label is applied to the content. This       label indicates that multiple containers can share the volume       content, for both reading and writing.     - `Z`: a _private unshared_ label is applied to the content.       This label indicates that only the current container can use       a private volume. Labeling systems such as SELinux require       proper labels to be placed on volume content that is mounted       into a container. Without a label, the security system can       prevent a container's processes from using the content. By       default, the labels set by the host operating system are not       modified. - `[[r]shared|[r]slave|[r]private]` specifies mount   [propagation behavior](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt).   This only applies to bind-mounted volumes, not internal volumes   or named volumes. Mount propagation requires the source mount   point (the location where the source directory is mounted in the   host operating system) to have the correct propagation properties.   For shared volumes, the source mount point must be set to `shared`.   For slave volumes, the mount must be set to either `shared` or   `slave`.
    #[serde(rename = "Binds", skip_serializing_if = "Option::is_none")]
    pub binds: Option<Vec<String>>,
    /// Path to a file where the container ID is written
    #[serde(rename = "ContainerIDFile", skip_serializing_if = "Option::is_none")]
    pub container_id_file: Option<String>,
    #[serde(rename = "LogConfig", skip_serializing_if = "Option::is_none")]
    pub log_config: Option<Box<models::HostConfigAllOfLogConfig>>,
    /// Network mode to use for this container. Supported standard values are: `bridge`, `host`, `none`, and `container:<name|id>`. Any other value is taken as a custom network's name to which this container should connect to.
    #[serde(rename = "NetworkMode", skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// PortMap describes the mapping of container ports to host ports, using the container's port-number and protocol as key in the format `<port>/<protocol>`, for example, `80/udp`.  If a container's port is mapped for multiple protocols, separate entries are added to the mapping table.
    #[serde(rename = "PortBindings", skip_serializing_if = "Option::is_none")]
    pub port_bindings: Option<
        std::collections::HashMap<String, Vec<models::PortBinding>>,
    >,
    #[serde(rename = "RestartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<Box<models::RestartPolicy>>,
    /// Automatically remove the container when the container's process exits. This has no effect if `RestartPolicy` is set.
    #[serde(rename = "AutoRemove", skip_serializing_if = "Option::is_none")]
    pub auto_remove: Option<bool>,
    /// Driver that this container uses to mount volumes.
    #[serde(rename = "VolumeDriver", skip_serializing_if = "Option::is_none")]
    pub volume_driver: Option<String>,
    /// A list of volumes to inherit from another container, specified in the form `<container name>[:<ro|rw>]`.
    #[serde(rename = "VolumesFrom", skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<String>>,
    /// Specification for mounts to be added to the container.
    #[serde(rename = "Mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::Mount>>,
    /// Initial console size, as an `[height, width]` array.
    #[serde(
        rename = "ConsoleSize",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub console_size: Option<Option<Vec<u32>>>,
    /// Arbitrary non-identifying metadata attached to container and provided to the runtime when the container is started.
    #[serde(rename = "Annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    /// A list of kernel capabilities to add to the container. Conflicts with option 'Capabilities'.
    #[serde(rename = "CapAdd", skip_serializing_if = "Option::is_none")]
    pub cap_add: Option<Vec<String>>,
    /// A list of kernel capabilities to drop from the container. Conflicts with option 'Capabilities'.
    #[serde(rename = "CapDrop", skip_serializing_if = "Option::is_none")]
    pub cap_drop: Option<Vec<String>>,
    /// cgroup namespace mode for the container. Possible values are:  - `\"private\"`: the container runs in its own private cgroup namespace - `\"host\"`: use the host system's cgroup namespace  If not specified, the daemon default is used, which can either be `\"private\"` or `\"host\"`, depending on daemon version, kernel support and configuration.
    #[serde(rename = "CgroupnsMode", skip_serializing_if = "Option::is_none")]
    pub cgroupns_mode: Option<CgroupnsMode>,
    /// A list of DNS servers for the container to use.
    #[serde(rename = "Dns", skip_serializing_if = "Option::is_none")]
    pub dns: Option<Vec<String>>,
    /// A list of DNS options.
    #[serde(rename = "DnsOptions", skip_serializing_if = "Option::is_none")]
    pub dns_options: Option<Vec<String>>,
    /// A list of DNS search domains.
    #[serde(rename = "DnsSearch", skip_serializing_if = "Option::is_none")]
    pub dns_search: Option<Vec<String>>,
    /// A list of hostnames/IP mappings to add to the container's `/etc/hosts` file. Specified in the form `[\"hostname:IP\"]`.
    #[serde(rename = "ExtraHosts", skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<String>>,
    /// A list of additional groups that the container process will run as.
    #[serde(rename = "GroupAdd", skip_serializing_if = "Option::is_none")]
    pub group_add: Option<Vec<String>>,
    /// IPC sharing mode for the container. Possible values are:  - `\"none\"`: own private IPC namespace, with /dev/shm not mounted - `\"private\"`: own private IPC namespace - `\"shareable\"`: own private IPC namespace, with a possibility to share it with other containers - `\"container:<name|id>\"`: join another (shareable) container's IPC namespace - `\"host\"`: use the host system's IPC namespace  If not specified, daemon default is used, which can either be `\"private\"` or `\"shareable\"`, depending on daemon version and configuration.
    #[serde(rename = "IpcMode", skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// Cgroup to use for the container.
    #[serde(rename = "Cgroup", skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<String>,
    /// A list of links for the container in the form `container_name:alias`.
    #[serde(rename = "Links", skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// An integer value containing the score given to the container in order to tune OOM killer preferences.
    #[serde(rename = "OomScoreAdj", skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i32>,
    /// Set the PID (Process) Namespace mode for the container. It can be either:  - `\"container:<name|id>\"`: joins another container's PID namespace - `\"host\"`: use the host's PID namespace inside the container
    #[serde(rename = "PidMode", skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// Gives the container full access to the host.
    #[serde(rename = "Privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// Allocates an ephemeral host port for all of a container's exposed ports.  Ports are de-allocated when the container stops and allocated when the container starts. The allocated port might be changed when restarting the container.  The port is selected from the ephemeral port range that depends on the kernel. For example, on Linux the range is defined by `/proc/sys/net/ipv4/ip_local_port_range`.
    #[serde(rename = "PublishAllPorts", skip_serializing_if = "Option::is_none")]
    pub publish_all_ports: Option<bool>,
    /// Mount the container's root filesystem as read only.
    #[serde(rename = "ReadonlyRootfs", skip_serializing_if = "Option::is_none")]
    pub readonly_rootfs: Option<bool>,
    /// A list of string values to customize labels for MLS systems, such as SELinux.
    #[serde(rename = "SecurityOpt", skip_serializing_if = "Option::is_none")]
    pub security_opt: Option<Vec<String>>,
    /// Storage driver options for this container, in the form `{\"size\": \"120G\"}`.
    #[serde(rename = "StorageOpt", skip_serializing_if = "Option::is_none")]
    pub storage_opt: Option<std::collections::HashMap<String, String>>,
    /// A map of container directories which should be replaced by tmpfs mounts, and their corresponding mount options. For example:  ``` { \"/run\": \"rw,noexec,nosuid,size=65536k\" } ```
    #[serde(rename = "Tmpfs", skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<std::collections::HashMap<String, String>>,
    /// UTS namespace to use for the container.
    #[serde(rename = "UTSMode", skip_serializing_if = "Option::is_none")]
    pub uts_mode: Option<String>,
    /// Sets the usernamespace mode for the container when usernamespace remapping option is enabled.
    #[serde(rename = "UsernsMode", skip_serializing_if = "Option::is_none")]
    pub userns_mode: Option<String>,
    /// Size of `/dev/shm` in bytes. If omitted, the system uses 64MB.
    #[serde(rename = "ShmSize", skip_serializing_if = "Option::is_none")]
    pub shm_size: Option<u64>,
    /// A list of kernel parameters (sysctls) to set in the container. For example:  ``` {\"net.ipv4.ip_forward\": \"1\"} ```
    #[serde(rename = "Sysctls", skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<std::collections::HashMap<String, String>>,
    /// Runtime to use with this container.
    #[serde(rename = "Runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// Isolation technology of the container. (Windows only)
    #[serde(rename = "Isolation", skip_serializing_if = "Option::is_none")]
    pub isolation: Option<Isolation>,
    /// The list of paths to be masked inside the container (this overrides the default set of paths).
    #[serde(rename = "MaskedPaths", skip_serializing_if = "Option::is_none")]
    pub masked_paths: Option<Vec<String>>,
    /// The list of paths to be set as read-only inside the container (this overrides the default set of paths).
    #[serde(rename = "ReadonlyPaths", skip_serializing_if = "Option::is_none")]
    pub readonly_paths: Option<Vec<String>>,
}
impl HostConfig {
    /// Container configuration that depends on the host we are running on
    pub fn new() -> HostConfig {
        HostConfig {
            cpu_shares: None,
            memory: None,
            cgroup_parent: None,
            blkio_weight: None,
            blkio_weight_device: None,
            blkio_device_read_bps: None,
            blkio_device_write_bps: None,
            blkio_device_read_i_ops: None,
            blkio_device_write_i_ops: None,
            cpu_period: None,
            cpu_quota: None,
            cpu_realtime_period: None,
            cpu_realtime_runtime: None,
            cpuset_cpus: None,
            cpuset_mems: None,
            devices: None,
            device_cgroup_rules: None,
            device_requests: None,
            kernel_memory_tcp: None,
            memory_reservation: None,
            memory_swap: None,
            memory_swappiness: None,
            nano_cpus: None,
            oom_kill_disable: None,
            init: None,
            pids_limit: None,
            ulimits: None,
            cpu_count: None,
            cpu_percent: None,
            io_maximum_i_ops: None,
            io_maximum_bandwidth: None,
            binds: None,
            container_id_file: None,
            log_config: None,
            network_mode: None,
            port_bindings: None,
            restart_policy: None,
            auto_remove: None,
            volume_driver: None,
            volumes_from: None,
            mounts: None,
            console_size: None,
            annotations: None,
            cap_add: None,
            cap_drop: None,
            cgroupns_mode: None,
            dns: None,
            dns_options: None,
            dns_search: None,
            extra_hosts: None,
            group_add: None,
            ipc_mode: None,
            cgroup: None,
            links: None,
            oom_score_adj: None,
            pid_mode: None,
            privileged: None,
            publish_all_ports: None,
            readonly_rootfs: None,
            security_opt: None,
            storage_opt: None,
            tmpfs: None,
            uts_mode: None,
            userns_mode: None,
            shm_size: None,
            sysctls: None,
            runtime: None,
            isolation: None,
            masked_paths: None,
            readonly_paths: None,
        }
    }
}
/// cgroup namespace mode for the container. Possible values are:  - `\"private\"`: the container runs in its own private cgroup namespace - `\"host\"`: use the host system's cgroup namespace  If not specified, the daemon default is used, which can either be `\"private\"` or `\"host\"`, depending on daemon version, kernel support and configuration.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum CgroupnsMode {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "host")]
    Host,
}
impl Default for CgroupnsMode {
    fn default() -> CgroupnsMode {
        Self::Private
    }
}
/// Isolation technology of the container. (Windows only)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Isolation {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "hyperv")]
    Hyperv,
    #[serde(rename = "")]
    Empty,
}
impl Default for Isolation {
    fn default() -> Isolation {
        Self::Default
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// HostConfigAllOfLogConfig : The logging configuration for this container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct HostConfigAllOfLogConfig {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,
}
impl HostConfigAllOfLogConfig {
    /// The logging configuration for this container
    pub fn new() -> HostConfigAllOfLogConfig {
        HostConfigAllOfLogConfig {
            r#type: None,
            config: None,
        }
    }
}
///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Type {
    #[serde(rename = "json-file")]
    JsonFile,
    #[serde(rename = "syslog")]
    Syslog,
    #[serde(rename = "journald")]
    Journald,
    #[serde(rename = "gelf")]
    Gelf,
    #[serde(rename = "fluentd")]
    Fluentd,
    #[serde(rename = "awslogs")]
    Awslogs,
    #[serde(rename = "splunk")]
    Splunk,
    #[serde(rename = "etwlogs")]
    Etwlogs,
    #[serde(rename = "none")]
    None,
}
impl Default for Type {
    fn default() -> Type {
        Self::JsonFile
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// IdResponse : Response to an API call that returns just an Id
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdResponse {
    /// The id of the newly created object.
    #[serde(rename = "Id")]
    pub id: String,
}
impl IdResponse {
    /// Response to an API call that returns just an Id
    pub fn new(id: String) -> IdResponse {
        IdResponse { id }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageConfig : Configuration of the image. These fields are used as defaults when starting a container from the image.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageConfig {
    /// The hostname to use for the container, as a valid RFC 1123 hostname.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always empty. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The domain name to use for the container.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always empty. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "Domainname", skip_serializing_if = "Option::is_none")]
    pub domainname: Option<String>,
    /// The user that commands are run as inside the container.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Whether to attach to `stdin`.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "AttachStdin", skip_serializing_if = "Option::is_none")]
    pub attach_stdin: Option<bool>,
    /// Whether to attach to `stdout`.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "AttachStdout", skip_serializing_if = "Option::is_none")]
    pub attach_stdout: Option<bool>,
    /// Whether to attach to `stderr`.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "AttachStderr", skip_serializing_if = "Option::is_none")]
    pub attach_stderr: Option<bool>,
    /// An object mapping ports to an empty object in the form:  `{\"<port>/<tcp|udp|sctp>\": {}}`
    #[serde(
        rename = "ExposedPorts",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub exposed_ports: Option<
        Option<std::collections::HashMap<String, serde_json::Value>>,
    >,
    /// Attach standard streams to a TTY, including `stdin` if it is not closed.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "Tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Open `stdin`  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Close `stdin` after one attached client disconnects.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always false. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "StdinOnce", skip_serializing_if = "Option::is_none")]
    pub stdin_once: Option<bool>,
    /// A list of environment variables to set inside the container in the form `[\"VAR=value\", ...]`. A variable without `=` is removed from the environment, rather than to have an empty value.
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// Command to run specified as a string or an array of strings.
    #[serde(rename = "Cmd", skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<String>>,
    #[serde(rename = "Healthcheck", skip_serializing_if = "Option::is_none")]
    pub healthcheck: Option<Box<models::HealthConfig>>,
    /// Command is already escaped (Windows only)
    #[serde(
        rename = "ArgsEscaped",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub args_escaped: Option<Option<bool>>,
    /// The name (or reference) of the image to use when creating the container, or which was used when the container was created.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always empty. It must not be used, and will be removed in API v1.48.
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// An object mapping mount point paths inside the container to empty objects.
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// The working directory for commands to run in.
    #[serde(rename = "WorkingDir", skip_serializing_if = "Option::is_none")]
    pub working_dir: Option<String>,
    /// The entry point for the container as a string or an array of strings.  If the array consists of exactly one empty string (`[\"\"]`) then the entry point is reset to system default (i.e., the entry point used by docker when there is no `ENTRYPOINT` instruction in the `Dockerfile`).
    #[serde(rename = "Entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Vec<String>>,
    /// Disable networking for the container.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always omitted. It must not be used, and will be removed in API v1.48.
    #[serde(
        rename = "NetworkDisabled",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_disabled: Option<Option<bool>>,
    /// MAC address of the container.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always omitted. It must not be used, and will be removed in API v1.48.
    #[serde(
        rename = "MacAddress",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub mac_address: Option<Option<String>>,
    /// `ONBUILD` metadata that were defined in the image's `Dockerfile`.
    #[serde(
        rename = "OnBuild",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub on_build: Option<Option<Vec<String>>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Signal to stop a container as a string or unsigned integer.
    #[serde(
        rename = "StopSignal",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_signal: Option<Option<String>>,
    /// Timeout to stop a container in seconds.  <p><br /></p>  > **Deprecated**: this field is not part of the image specification and is > always omitted. It must not be used, and will be removed in API v1.48.
    #[serde(
        rename = "StopTimeout",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_timeout: Option<Option<i32>>,
    /// Shell for when `RUN`, `CMD`, and `ENTRYPOINT` uses a shell.
    #[serde(
        rename = "Shell",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub shell: Option<Option<Vec<String>>>,
}
impl ImageConfig {
    /// Configuration of the image. These fields are used as defaults when starting a container from the image.
    pub fn new() -> ImageConfig {
        ImageConfig {
            hostname: None,
            domainname: None,
            user: None,
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            exposed_ports: None,
            tty: None,
            open_stdin: None,
            stdin_once: None,
            env: None,
            cmd: None,
            healthcheck: None,
            args_escaped: None,
            image: None,
            volumes: None,
            working_dir: None,
            entrypoint: None,
            network_disabled: None,
            mac_address: None,
            on_build: None,
            labels: None,
            stop_signal: None,
            stop_timeout: None,
            shell: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageDeleteResponseItem {
    /// The image ID of an image that was untagged
    #[serde(rename = "Untagged", skip_serializing_if = "Option::is_none")]
    pub untagged: Option<String>,
    /// The image ID of an image that was deleted
    #[serde(rename = "Deleted", skip_serializing_if = "Option::is_none")]
    pub deleted: Option<String>,
}
impl ImageDeleteResponseItem {
    pub fn new() -> ImageDeleteResponseItem {
        ImageDeleteResponseItem {
            untagged: None,
            deleted: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageId : Image ID or Digest
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageId {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
impl ImageId {
    /// Image ID or Digest
    pub fn new() -> ImageId {
        ImageId { id: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageInspect : Information about an image in the local image cache.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInspect {
    /// ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.
    #[serde(rename = "RepoTags", skip_serializing_if = "Option::is_none")]
    pub repo_tags: Option<Vec<String>>,
    /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
    #[serde(rename = "RepoDigests", skip_serializing_if = "Option::is_none")]
    pub repo_digests: Option<Vec<String>>,
    /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
    #[serde(rename = "Parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Optional message that was set when committing or importing the image.
    #[serde(rename = "Comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Date and time at which the image was created, formatted in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if present in the image, and omitted otherwise.
    #[serde(
        rename = "Created",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<Option<String>>,
    /// The version of Docker that was used to build the image.  Depending on how the image was created, this field may be empty.
    #[serde(rename = "DockerVersion", skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    /// Name of the author that was specified when committing the image, or as specified through MAINTAINER (deprecated) in the Dockerfile.
    #[serde(rename = "Author", skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Box<models::ImageConfig>>,
    /// Hardware CPU architecture that the image runs on.
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// CPU architecture variant (presently ARM-only).
    #[serde(
        rename = "Variant",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub variant: Option<Option<String>>,
    /// Operating System the image is built to run on.
    #[serde(rename = "Os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Operating System version the image is built to run on (especially for Windows).
    #[serde(
        rename = "OsVersion",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub os_version: Option<Option<String>>,
    /// Total size of the image including all layers it is composed of.
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// Total size of the image including all layers it is composed of.  Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    #[serde(rename = "VirtualSize", skip_serializing_if = "Option::is_none")]
    pub virtual_size: Option<i64>,
    #[serde(rename = "GraphDriver", skip_serializing_if = "Option::is_none")]
    pub graph_driver: Option<Box<models::DriverData>>,
    #[serde(rename = "RootFS", skip_serializing_if = "Option::is_none")]
    pub root_fs: Option<Box<models::ImageInspectRootFs>>,
    #[serde(rename = "Metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Box<models::ImageInspectMetadata>>,
}
impl ImageInspect {
    /// Information about an image in the local image cache.
    pub fn new() -> ImageInspect {
        ImageInspect {
            id: None,
            repo_tags: None,
            repo_digests: None,
            parent: None,
            comment: None,
            created: None,
            docker_version: None,
            author: None,
            config: None,
            architecture: None,
            variant: None,
            os: None,
            os_version: None,
            size: None,
            virtual_size: None,
            graph_driver: None,
            root_fs: None,
            metadata: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageInspectMetadata : Additional metadata of the image in the local cache. This information is local to the daemon, and not part of the image itself.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInspectMetadata {
    /// Date and time at which the image was last tagged in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.  This information is only available if the image was tagged locally, and omitted otherwise.
    #[serde(
        rename = "LastTagTime",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_tag_time: Option<Option<String>>,
}
impl ImageInspectMetadata {
    /// Additional metadata of the image in the local cache. This information is local to the daemon, and not part of the image itself.
    pub fn new() -> ImageInspectMetadata {
        ImageInspectMetadata {
            last_tag_time: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageInspectRootFs : Information about the image's RootFS, including the layer IDs.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageInspectRootFs {
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Layers", skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
}
impl ImageInspectRootFs {
    /// Information about the image's RootFS, including the layer IDs.
    pub fn new(r#type: String) -> ImageInspectRootFs {
        ImageInspectRootFs {
            r#type,
            layers: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageManifestSummary : ImageManifestSummary represents a summary of an image manifest.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageManifestSummary {
    /// ID is the content-addressable ID of an image and is the same as the digest of the image manifest.
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "Descriptor")]
    pub descriptor: Box<models::OciDescriptor>,
    /// Indicates whether all the child content (image config, layers) is fully available locally.
    #[serde(rename = "Available")]
    pub available: bool,
    #[serde(rename = "Size")]
    pub size: Box<models::ImageManifestSummarySize>,
    /// The kind of the manifest.  kind         | description -------------|----------------------------------------------------------- image        | Image manifest that can be used to start a container. attestation  | Attestation manifest produced by the Buildkit builder for a specific image manifest.
    #[serde(rename = "Kind")]
    pub kind: Kind,
    #[serde(
        rename = "ImageData",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_data: Option<Option<Box<models::ImageManifestSummaryImageData>>>,
    #[serde(
        rename = "AttestationData",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub attestation_data: Option<
        Option<Box<models::ImageManifestSummaryAttestationData>>,
    >,
}
impl ImageManifestSummary {
    /// ImageManifestSummary represents a summary of an image manifest.
    pub fn new(
        id: String,
        descriptor: models::OciDescriptor,
        available: bool,
        size: models::ImageManifestSummarySize,
        kind: Kind,
    ) -> ImageManifestSummary {
        ImageManifestSummary {
            id,
            descriptor: Box::new(descriptor),
            available,
            size: Box::new(size),
            kind,
            image_data: None,
            attestation_data: None,
        }
    }
}
/// The kind of the manifest.  kind         | description -------------|----------------------------------------------------------- image        | Image manifest that can be used to start a container. attestation  | Attestation manifest produced by the Buildkit builder for a specific image manifest.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Kind {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "attestation")]
    Attestation,
    #[serde(rename = "unknown")]
    Unknown,
}
impl Default for Kind {
    fn default() -> Kind {
        Self::Image
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageManifestSummaryAttestationData : The image data for the attestation manifest. This field is only populated when Kind is \"attestation\".
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageManifestSummaryAttestationData {
    /// The digest of the image manifest that this attestation is for.
    #[serde(rename = "For")]
    pub r#for: String,
}
impl ImageManifestSummaryAttestationData {
    /// The image data for the attestation manifest. This field is only populated when Kind is \"attestation\".
    pub fn new(r#for: String) -> ImageManifestSummaryAttestationData {
        ImageManifestSummaryAttestationData {
            r#for,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ImageManifestSummaryImageData : The image data for the image manifest. This field is only populated when Kind is \"image\".
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageManifestSummaryImageData {
    #[serde(rename = "Platform")]
    pub platform: Box<models::OciPlatform>,
    /// The IDs of the containers that are using this image.
    #[serde(rename = "Containers")]
    pub containers: Vec<String>,
    #[serde(rename = "Size")]
    pub size: Box<models::ImageManifestSummaryImageDataSize>,
}
impl ImageManifestSummaryImageData {
    /// The image data for the image manifest. This field is only populated when Kind is \"image\".
    pub fn new(
        platform: models::OciPlatform,
        containers: Vec<String>,
        size: models::ImageManifestSummaryImageDataSize,
    ) -> ImageManifestSummaryImageData {
        ImageManifestSummaryImageData {
            platform: Box::new(platform),
            containers,
            size: Box::new(size),
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageManifestSummaryImageDataSize {
    /// Unpacked is the size (in bytes) of the locally unpacked (uncompressed) image content that's directly usable by the containers running this image. It's independent of the distributable content - e.g. the image might still have an unpacked data that's still used by some container even when the distributable/compressed content is already gone.
    #[serde(rename = "Unpacked")]
    pub unpacked: i64,
}
impl ImageManifestSummaryImageDataSize {
    pub fn new(unpacked: i64) -> ImageManifestSummaryImageDataSize {
        ImageManifestSummaryImageDataSize {
            unpacked,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageManifestSummarySize {
    /// Total is the total size (in bytes) of all the locally present data (both distributable and non-distributable) that's related to this manifest and its children. This equal to the sum of [Content] size AND all the sizes in the [Size] struct present in the Kind-specific data struct. For example, for an image kind (Kind == \"image\") this would include the size of the image content and unpacked image snapshots ([Size.Content] + [ImageData.Size.Unpacked]).
    #[serde(rename = "Total")]
    pub total: i64,
    /// Content is the size (in bytes) of all the locally present content in the content store (e.g. image config, layers) referenced by this manifest and its children. This only includes blobs in the content store.
    #[serde(rename = "Content")]
    pub content: i64,
}
impl ImageManifestSummarySize {
    pub fn new(total: i64, content: i64) -> ImageManifestSummarySize {
        ImageManifestSummarySize {
            total,
            content,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImagePruneResponse {
    /// Images that were deleted
    #[serde(rename = "ImagesDeleted", skip_serializing_if = "Option::is_none")]
    pub images_deleted: Option<Vec<models::ImageDeleteResponseItem>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed", skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
impl ImagePruneResponse {
    pub fn new() -> ImagePruneResponse {
        ImagePruneResponse {
            images_deleted: None,
            space_reclaimed: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageSearchResponseItem {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "is_official", skip_serializing_if = "Option::is_none")]
    pub is_official: Option<bool>,
    /// Whether this repository has automated builds enabled.  <p><br /></p>  > **Deprecated**: This field is deprecated and will always be \"false\".
    #[serde(rename = "is_automated", skip_serializing_if = "Option::is_none")]
    pub is_automated: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "star_count", skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i32>,
}
impl ImageSearchResponseItem {
    pub fn new() -> ImageSearchResponseItem {
        ImageSearchResponseItem {
            description: None,
            is_official: None,
            is_automated: None,
            name: None,
            star_count: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageSummary {
    /// ID is the content-addressable ID of an image.  This identifier is a content-addressable digest calculated from the image's configuration (which includes the digests of layers used by the image).  Note that this digest differs from the `RepoDigests` below, which holds digests of image manifests that reference the image.
    #[serde(rename = "Id")]
    pub id: String,
    /// ID of the parent image.  Depending on how the image was created, this field may be empty and is only set for images that were built/created locally. This field is empty if the image was pulled from an image registry.
    #[serde(rename = "ParentId")]
    pub parent_id: String,
    /// List of image names/tags in the local image cache that reference this image.  Multiple image tags can refer to the same image, and this list may be empty if no tags reference the image, in which case the image is \"untagged\", in which case it can still be referenced by its ID.
    #[serde(rename = "RepoTags")]
    pub repo_tags: Vec<String>,
    /// List of content-addressable digests of locally available image manifests that the image is referenced from. Multiple manifests can refer to the same image.  These digests are usually only available if the image was either pulled from a registry, or if the image was pushed to a registry, which is when the manifest is generated and its digest calculated.
    #[serde(rename = "RepoDigests")]
    pub repo_digests: Vec<String>,
    /// Date and time at which the image was created as a Unix timestamp (number of seconds since EPOCH).
    #[serde(rename = "Created")]
    pub created: i32,
    /// Total size of the image including all layers it is composed of.
    #[serde(rename = "Size")]
    pub size: i64,
    /// Total size of image layers that are shared between this image and other images.  This size is not calculated by default. `-1` indicates that the value has not been set / calculated.
    #[serde(rename = "SharedSize")]
    pub shared_size: i64,
    /// Total size of the image including all layers it is composed of.  Deprecated: this field is omitted in API v1.44, but kept for backward compatibility. Use Size instead.
    #[serde(rename = "VirtualSize", skip_serializing_if = "Option::is_none")]
    pub virtual_size: Option<i64>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    pub labels: std::collections::HashMap<String, String>,
    /// Number of containers using this image. Includes both stopped and running containers.  This size is not calculated by default, and depends on which API endpoint is used. `-1` indicates that the value has not been set / calculated.
    #[serde(rename = "Containers")]
    pub containers: i32,
    /// Manifests is a list of manifests available in this image. It provides a more detailed view of the platform-specific image manifests or other image-attached data like build attestations.  WARNING: This is experimental and may change at any time without any backward compatibility.
    #[serde(rename = "Manifests", skip_serializing_if = "Option::is_none")]
    pub manifests: Option<Vec<models::ImageManifestSummary>>,
}
impl ImageSummary {
    pub fn new(
        id: String,
        parent_id: String,
        repo_tags: Vec<String>,
        repo_digests: Vec<String>,
        created: i32,
        size: i64,
        shared_size: i64,
        labels: std::collections::HashMap<String, String>,
        containers: i32,
    ) -> ImageSummary {
        ImageSummary {
            id,
            parent_id,
            repo_tags,
            repo_digests,
            created,
            size,
            shared_size,
            virtual_size: None,
            labels,
            containers,
            manifests: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// IndexInfo : IndexInfo contains information about a registry.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndexInfo {
    /// Name of the registry, such as \"docker.io\".
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// List of mirrors, expressed as URIs.
    #[serde(rename = "Mirrors", skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
    /// Indicates if the registry is part of the list of insecure registries.  If `false`, the registry is insecure. Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  > **Warning**: Insecure registries can be useful when running a local > registry. However, because its use creates security vulnerabilities > it should ONLY be enabled for testing purposes. For increased > security, users should add their CA to their system's list of > trusted CAs instead of enabling this option.
    #[serde(rename = "Secure", skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    /// Indicates whether this is an official registry (i.e., Docker Hub / docker.io)
    #[serde(rename = "Official", skip_serializing_if = "Option::is_none")]
    pub official: Option<bool>,
}
impl IndexInfo {
    /// IndexInfo contains information about a registry.
    pub fn new() -> IndexInfo {
        IndexInfo {
            name: None,
            mirrors: None,
            secure: None,
            official: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ipam {
    /// Name of the IPAM driver to use.
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// List of IPAM configuration options, specified as a map:  ``` {\"Subnet\": <CIDR>, \"IPRange\": <CIDR>, \"Gateway\": <IP address>, \"AuxAddress\": <device_name:IP address>} ```
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<Vec<models::IpamConfig>>,
    /// Driver-specific options, specified as a map.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
}
impl Ipam {
    pub fn new() -> Ipam {
        Ipam {
            driver: None,
            config: None,
            options: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IpamConfig {
    #[serde(rename = "Subnet", skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    #[serde(rename = "IPRange", skip_serializing_if = "Option::is_none")]
    pub ip_range: Option<String>,
    #[serde(rename = "Gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    #[serde(rename = "AuxiliaryAddresses", skip_serializing_if = "Option::is_none")]
    pub auxiliary_addresses: Option<std::collections::HashMap<String, String>>,
}
impl IpamConfig {
    pub fn new() -> IpamConfig {
        IpamConfig {
            subnet: None,
            ip_range: None,
            gateway: None,
            auxiliary_addresses: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// JoinTokens : JoinTokens contains the tokens workers and managers need to join the swarm.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JoinTokens {
    /// The token workers can use to join the swarm.
    #[serde(rename = "Worker", skip_serializing_if = "Option::is_none")]
    pub worker: Option<String>,
    /// The token managers can use to join the swarm.
    #[serde(rename = "Manager", skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
}
impl JoinTokens {
    /// JoinTokens contains the tokens workers and managers need to join the swarm.
    pub fn new() -> JoinTokens {
        JoinTokens {
            worker: None,
            manager: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Limit : An object describing a limit on resources which can be requested by a task.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Limit {
    #[serde(rename = "NanoCPUs", skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "MemoryBytes", skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<i64>,
    /// Limits the maximum number of PIDs in the container. Set `0` for unlimited.
    #[serde(rename = "Pids", skip_serializing_if = "Option::is_none")]
    pub pids: Option<i64>,
}
impl Limit {
    /// An object describing a limit on resources which can be requested by a task.
    pub fn new() -> Limit {
        Limit {
            nano_cpus: None,
            memory_bytes: None,
            pids: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// LocalNodeState : Current local status of this node.
/// Current local status of this node.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum LocalNodeState {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "locked")]
    Locked,
}
impl std::fmt::Display for LocalNodeState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, ""),
            Self::Inactive => write!(f, "inactive"),
            Self::Pending => write!(f, "pending"),
            Self::Active => write!(f, "active"),
            Self::Error => write!(f, "error"),
            Self::Locked => write!(f, "locked"),
        }
    }
}
impl Default for LocalNodeState {
    fn default() -> LocalNodeState {
        Self::Empty
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ManagerStatus : ManagerStatus represents the status of a manager.  It provides the current status of a node's manager component, if the node is a manager.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ManagerStatus {
    #[serde(rename = "Leader", skip_serializing_if = "Option::is_none")]
    pub leader: Option<bool>,
    #[serde(rename = "Reachability", skip_serializing_if = "Option::is_none")]
    pub reachability: Option<models::Reachability>,
    /// The IP address and port at which the manager is reachable.
    #[serde(rename = "Addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
impl ManagerStatus {
    /// ManagerStatus represents the status of a manager.  It provides the current status of a node's manager component, if the node is a manager.
    pub fn new() -> ManagerStatus {
        ManagerStatus {
            leader: None,
            reachability: None,
            addr: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mount {
    /// Container path.
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Mount source (e.g. a volume name, a host path).
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The mount type. Available types:  - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container. - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed. - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs. - `npipe` Mounts a named pipe from the host into the container. Must exist prior to creating the container. - `cluster` a Swarm cluster volume
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Whether the mount should be read-only.
    #[serde(rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// The consistency requirement for the mount: `default`, `consistent`, `cached`, or `delegated`.
    #[serde(rename = "Consistency", skip_serializing_if = "Option::is_none")]
    pub consistency: Option<String>,
    #[serde(rename = "BindOptions", skip_serializing_if = "Option::is_none")]
    pub bind_options: Option<Box<models::MountBindOptions>>,
    #[serde(rename = "VolumeOptions", skip_serializing_if = "Option::is_none")]
    pub volume_options: Option<Box<models::MountVolumeOptions>>,
    #[serde(rename = "TmpfsOptions", skip_serializing_if = "Option::is_none")]
    pub tmpfs_options: Option<Box<models::MountTmpfsOptions>>,
}
impl Mount {
    pub fn new() -> Mount {
        Mount {
            target: None,
            source: None,
            r#type: None,
            read_only: None,
            consistency: None,
            bind_options: None,
            volume_options: None,
            tmpfs_options: None,
        }
    }
}
/// The mount type. Available types:  - `bind` Mounts a file or directory from the host into the container. Must exist prior to creating the container. - `volume` Creates a volume with the given name and options (or uses a pre-existing volume with the same name and options). These are **not** removed when the container is removed. - `tmpfs` Create a tmpfs with the given options. The mount source cannot be specified for tmpfs. - `npipe` Mounts a named pipe from the host into the container. Must exist prior to creating the container. - `cluster` a Swarm cluster volume
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Type {
    #[serde(rename = "bind")]
    Bind,
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "tmpfs")]
    Tmpfs,
    #[serde(rename = "npipe")]
    Npipe,
    #[serde(rename = "cluster")]
    Cluster,
}
impl Default for Type {
    fn default() -> Type {
        Self::Bind
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// MountBindOptions : Optional configuration for the `bind` type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountBindOptions {
    /// A propagation mode with the value `[r]private`, `[r]shared`, or `[r]slave`.
    #[serde(rename = "Propagation", skip_serializing_if = "Option::is_none")]
    pub propagation: Option<Propagation>,
    /// Disable recursive bind mount.
    #[serde(rename = "NonRecursive", skip_serializing_if = "Option::is_none")]
    pub non_recursive: Option<bool>,
    /// Create mount point on host if missing
    #[serde(rename = "CreateMountpoint", skip_serializing_if = "Option::is_none")]
    pub create_mountpoint: Option<bool>,
    /// Make the mount non-recursively read-only, but still leave the mount recursive (unless NonRecursive is set to `true` in conjunction).  Added in v1.44, before that version all read-only mounts were non-recursive by default. To match the previous behaviour this will default to `true` for clients on versions prior to v1.44.
    #[serde(rename = "ReadOnlyNonRecursive", skip_serializing_if = "Option::is_none")]
    pub read_only_non_recursive: Option<bool>,
    /// Raise an error if the mount cannot be made recursively read-only.
    #[serde(rename = "ReadOnlyForceRecursive", skip_serializing_if = "Option::is_none")]
    pub read_only_force_recursive: Option<bool>,
}
impl MountBindOptions {
    /// Optional configuration for the `bind` type.
    pub fn new() -> MountBindOptions {
        MountBindOptions {
            propagation: None,
            non_recursive: None,
            create_mountpoint: None,
            read_only_non_recursive: None,
            read_only_force_recursive: None,
        }
    }
}
/// A propagation mode with the value `[r]private`, `[r]shared`, or `[r]slave`.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Propagation {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "rprivate")]
    Rprivate,
    #[serde(rename = "shared")]
    Shared,
    #[serde(rename = "rshared")]
    Rshared,
    #[serde(rename = "slave")]
    Slave,
    #[serde(rename = "rslave")]
    Rslave,
}
impl Default for Propagation {
    fn default() -> Propagation {
        Self::Private
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// MountPoint : MountPoint represents a mount point configuration inside the container. This is used for reporting the mountpoints in use by a container.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountPoint {
    /// The mount type:  - `bind` a mount of a file or directory from the host into the container. - `volume` a docker volume with the given `Name`. - `tmpfs` a `tmpfs`. - `npipe` a named pipe from the host into the container. - `cluster` a Swarm cluster volume
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    /// Name is the name reference to the underlying data defined by `Source` e.g., the volume name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Source location of the mount.  For volumes, this contains the storage location of the volume (within `/var/lib/docker/volumes/`). For bind-mounts, and `npipe`, this contains the source (host) part of the bind-mount. For `tmpfs` mount points, this field is empty.
    #[serde(rename = "Source", skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Destination is the path relative to the container root (`/`) where the `Source` is mounted inside the container.
    #[serde(rename = "Destination", skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// Driver is the volume driver used to create the volume (if it is a volume).
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Mode is a comma separated list of options supplied by the user when creating the bind/volume mount.  The default is platform-specific (`\"z\"` on Linux, empty on Windows).
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// Whether the mount is mounted writable (read-write).
    #[serde(rename = "RW", skip_serializing_if = "Option::is_none")]
    pub rw: Option<bool>,
    /// Propagation describes how mounts are propagated from the host into the mount point, and vice-versa. Refer to the [Linux kernel documentation](https://www.kernel.org/doc/Documentation/filesystems/sharedsubtree.txt) for details. This field is not used on Windows.
    #[serde(rename = "Propagation", skip_serializing_if = "Option::is_none")]
    pub propagation: Option<String>,
}
impl MountPoint {
    /// MountPoint represents a mount point configuration inside the container. This is used for reporting the mountpoints in use by a container.
    pub fn new() -> MountPoint {
        MountPoint {
            r#type: None,
            name: None,
            source: None,
            destination: None,
            driver: None,
            mode: None,
            rw: None,
            propagation: None,
        }
    }
}
/// The mount type:  - `bind` a mount of a file or directory from the host into the container. - `volume` a docker volume with the given `Name`. - `tmpfs` a `tmpfs`. - `npipe` a named pipe from the host into the container. - `cluster` a Swarm cluster volume
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Type {
    #[serde(rename = "bind")]
    Bind,
    #[serde(rename = "volume")]
    Volume,
    #[serde(rename = "tmpfs")]
    Tmpfs,
    #[serde(rename = "npipe")]
    Npipe,
    #[serde(rename = "cluster")]
    Cluster,
}
impl Default for Type {
    fn default() -> Type {
        Self::Bind
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// MountTmpfsOptions : Optional configuration for the `tmpfs` type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountTmpfsOptions {
    /// The size for the tmpfs mount in bytes.
    #[serde(rename = "SizeBytes", skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    /// The permission mode for the tmpfs mount in an integer.
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
    /// The options to be passed to the tmpfs mount. An array of arrays. Flag options should be provided as 1-length arrays. Other types should be provided as as 2-length arrays, where the first item is the key and the second the value.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<Vec<String>>>,
}
impl MountTmpfsOptions {
    /// Optional configuration for the `tmpfs` type.
    pub fn new() -> MountTmpfsOptions {
        MountTmpfsOptions {
            size_bytes: None,
            mode: None,
            options: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// MountVolumeOptions : Optional configuration for the `volume` type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountVolumeOptions {
    /// Populate volume with data from the target.
    #[serde(rename = "NoCopy", skip_serializing_if = "Option::is_none")]
    pub no_copy: Option<bool>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DriverConfig", skip_serializing_if = "Option::is_none")]
    pub driver_config: Option<Box<models::MountVolumeOptionsDriverConfig>>,
    /// Source path inside the volume. Must be relative without any back traversals.
    #[serde(rename = "Subpath", skip_serializing_if = "Option::is_none")]
    pub subpath: Option<String>,
}
impl MountVolumeOptions {
    /// Optional configuration for the `volume` type.
    pub fn new() -> MountVolumeOptions {
        MountVolumeOptions {
            no_copy: None,
            labels: None,
            driver_config: None,
            subpath: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// MountVolumeOptionsDriverConfig : Map of driver specific options
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MountVolumeOptionsDriverConfig {
    /// Name of the driver to use to create the volume.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// key/value map of driver specific options.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
}
impl MountVolumeOptionsDriverConfig {
    /// Map of driver specific options
    pub fn new() -> MountVolumeOptionsDriverConfig {
        MountVolumeOptionsDriverConfig {
            name: None,
            options: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Network {
    /// Name of the network.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// ID that uniquely identifies a network on a single machine.
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Date and time at which the network was created in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "Created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The level at which the network exists (e.g. `swarm` for cluster-wide or `local` for machine level)
    #[serde(rename = "Scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// The name of the driver used to create the network (e.g. `bridge`, `overlay`).
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Whether the network was created with IPv4 enabled.
    #[serde(rename = "EnableIPv4", skip_serializing_if = "Option::is_none")]
    pub enable_ipv4: Option<bool>,
    /// Whether the network was created with IPv6 enabled.
    #[serde(rename = "EnableIPv6", skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    #[serde(rename = "IPAM", skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Box<models::Ipam>>,
    /// Whether the network is created to only allow internal networking connectivity.
    #[serde(rename = "Internal", skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Whether a global / swarm scope network is manually attachable by regular containers from workers in swarm mode.
    #[serde(rename = "Attachable", skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    /// Whether the network is providing the routing-mesh for the swarm cluster.
    #[serde(rename = "Ingress", skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    #[serde(rename = "ConfigFrom", skip_serializing_if = "Option::is_none")]
    pub config_from: Option<Box<models::ConfigReference>>,
    /// Whether the network is a config-only network. Config-only networks are placeholder networks for network configurations to be used by other networks. Config-only networks cannot be used directly to run containers or services.
    #[serde(rename = "ConfigOnly", skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    /// Contains endpoints attached to the network.
    #[serde(rename = "Containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<std::collections::HashMap<String, models::NetworkContainer>>,
    /// Network-specific options uses when creating the network.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// List of peer nodes for an overlay network. This field is only present for overlay networks, and omitted for other network types.
    #[serde(
        rename = "Peers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub peers: Option<Option<Vec<models::PeerInfo>>>,
}
impl Network {
    pub fn new() -> Network {
        Network {
            name: None,
            id: None,
            created: None,
            scope: None,
            driver: None,
            enable_ipv4: None,
            enable_ipv6: None,
            ipam: None,
            internal: None,
            attachable: None,
            ingress: None,
            config_from: None,
            config_only: None,
            containers: None,
            options: None,
            labels: None,
            peers: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// NetworkAttachmentConfig : Specifies how a service should be attached to a particular network.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkAttachmentConfig {
    /// The target network for attachment. Must be a network name or ID.
    #[serde(rename = "Target", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Discoverable alternate names for the service on this network.
    #[serde(rename = "Aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// Driver attachment options for the network target.
    #[serde(rename = "DriverOpts", skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
}
impl NetworkAttachmentConfig {
    /// Specifies how a service should be attached to a particular network.
    pub fn new() -> NetworkAttachmentConfig {
        NetworkAttachmentConfig {
            target: None,
            aliases: None,
            driver_opts: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkConnectRequest {
    /// The ID or name of the container to connect to the network.
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "EndpointConfig", skip_serializing_if = "Option::is_none")]
    pub endpoint_config: Option<Box<models::EndpointSettings>>,
}
impl NetworkConnectRequest {
    pub fn new() -> NetworkConnectRequest {
        NetworkConnectRequest {
            container: None,
            endpoint_config: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkContainer {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "EndpointID", skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(rename = "IPv4Address", skip_serializing_if = "Option::is_none")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "IPv6Address", skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
}
impl NetworkContainer {
    pub fn new() -> NetworkContainer {
        NetworkContainer {
            name: None,
            endpoint_id: None,
            mac_address: None,
            ipv4_address: None,
            ipv6_address: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkCreateRequest {
    /// The network's name.
    #[serde(rename = "Name")]
    pub name: String,
    /// Name of the network driver plugin to use.
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// The level at which the network exists (e.g. `swarm` for cluster-wide or `local` for machine level).
    #[serde(rename = "Scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    /// Restrict external access to the network.
    #[serde(rename = "Internal", skip_serializing_if = "Option::is_none")]
    pub internal: Option<bool>,
    /// Globally scoped network is manually attachable by regular containers from workers in swarm mode.
    #[serde(rename = "Attachable", skip_serializing_if = "Option::is_none")]
    pub attachable: Option<bool>,
    /// Ingress network is the network which provides the routing-mesh in swarm mode.
    #[serde(rename = "Ingress", skip_serializing_if = "Option::is_none")]
    pub ingress: Option<bool>,
    /// Creates a config-only network. Config-only networks are placeholder networks for network configurations to be used by other networks. Config-only networks cannot be used directly to run containers or services.
    #[serde(rename = "ConfigOnly", skip_serializing_if = "Option::is_none")]
    pub config_only: Option<bool>,
    #[serde(rename = "ConfigFrom", skip_serializing_if = "Option::is_none")]
    pub config_from: Option<Box<models::ConfigReference>>,
    #[serde(rename = "IPAM", skip_serializing_if = "Option::is_none")]
    pub ipam: Option<Box<models::Ipam>>,
    /// Enable IPv4 on the network. To disable IPv4, the daemon must be started with experimental features enabled.
    #[serde(rename = "EnableIPv4", skip_serializing_if = "Option::is_none")]
    pub enable_ipv4: Option<bool>,
    /// Enable IPv6 on the network.
    #[serde(rename = "EnableIPv6", skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// Network specific options to be used by the drivers.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
}
impl NetworkCreateRequest {
    pub fn new(name: String) -> NetworkCreateRequest {
        NetworkCreateRequest {
            name,
            driver: None,
            scope: None,
            internal: None,
            attachable: None,
            ingress: None,
            config_only: None,
            config_from: None,
            ipam: None,
            enable_ipv4: None,
            enable_ipv6: None,
            options: None,
            labels: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// NetworkCreateResponse : OK response to NetworkCreate operation
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkCreateResponse {
    /// The ID of the created network.
    #[serde(rename = "Id")]
    pub id: String,
    /// Warnings encountered when creating the container
    #[serde(rename = "Warning")]
    pub warning: String,
}
impl NetworkCreateResponse {
    /// OK response to NetworkCreate operation
    pub fn new(id: String, warning: String) -> NetworkCreateResponse {
        NetworkCreateResponse {
            id,
            warning,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkDisconnectRequest {
    /// The ID or name of the container to disconnect from the network.
    #[serde(rename = "Container", skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// Force the container to disconnect from the network.
    #[serde(rename = "Force", skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}
impl NetworkDisconnectRequest {
    pub fn new() -> NetworkDisconnectRequest {
        NetworkDisconnectRequest {
            container: None,
            force: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkPruneResponse {
    /// Networks that were deleted
    #[serde(rename = "NetworksDeleted", skip_serializing_if = "Option::is_none")]
    pub networks_deleted: Option<Vec<String>>,
}
impl NetworkPruneResponse {
    pub fn new() -> NetworkPruneResponse {
        NetworkPruneResponse {
            networks_deleted: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// NetworkSettings : NetworkSettings exposes the network settings in the API
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkSettings {
    /// Name of the default bridge interface when dockerd's --bridge flag is set.
    #[serde(rename = "Bridge", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
    /// SandboxID uniquely represents a container's network stack.
    #[serde(rename = "SandboxID", skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    /// Indicates if hairpin NAT should be enabled on the virtual interface.  Deprecated: This field is never set and will be removed in a future release.
    #[serde(rename = "HairpinMode", skip_serializing_if = "Option::is_none")]
    pub hairpin_mode: Option<bool>,
    /// IPv6 unicast address using the link-local prefix.  Deprecated: This field is never set and will be removed in a future release.
    #[serde(rename = "LinkLocalIPv6Address", skip_serializing_if = "Option::is_none")]
    pub link_local_ipv6_address: Option<String>,
    /// Prefix length of the IPv6 unicast address.  Deprecated: This field is never set and will be removed in a future release.
    #[serde(rename = "LinkLocalIPv6PrefixLen", skip_serializing_if = "Option::is_none")]
    pub link_local_ipv6_prefix_len: Option<i32>,
    /// PortMap describes the mapping of container ports to host ports, using the container's port-number and protocol as key in the format `<port>/<protocol>`, for example, `80/udp`.  If a container's port is mapped for multiple protocols, separate entries are added to the mapping table.
    #[serde(rename = "Ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<std::collections::HashMap<String, Vec<models::PortBinding>>>,
    /// SandboxKey is the full path of the netns handle
    #[serde(rename = "SandboxKey", skip_serializing_if = "Option::is_none")]
    pub sandbox_key: Option<String>,
    /// Deprecated: This field is never set and will be removed in a future release.
    #[serde(
        rename = "SecondaryIPAddresses",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_ip_addresses: Option<Option<Vec<models::Address>>>,
    /// Deprecated: This field is never set and will be removed in a future release.
    #[serde(
        rename = "SecondaryIPv6Addresses",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_ipv6_addresses: Option<Option<Vec<models::Address>>>,
    /// EndpointID uniquely represents a service endpoint in a Sandbox.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "EndpointID", skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    /// Gateway address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "Gateway", skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// Global IPv6 address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "GlobalIPv6Address", skip_serializing_if = "Option::is_none")]
    pub global_ipv6_address: Option<String>,
    /// Mask length of the global IPv6 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "GlobalIPv6PrefixLen", skip_serializing_if = "Option::is_none")]
    pub global_ipv6_prefix_len: Option<i32>,
    /// IPv4 address for the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "IPAddress", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Mask length of the IPv4 address.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "IPPrefixLen", skip_serializing_if = "Option::is_none")]
    pub ip_prefix_len: Option<i32>,
    /// IPv6 gateway address for this network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "IPv6Gateway", skip_serializing_if = "Option::is_none")]
    pub ipv6_gateway: Option<String>,
    /// MAC address for the container on the default \"bridge\" network.  <p><br /></p>  > **Deprecated**: This field is only propagated when attached to the > default \"bridge\" network. Use the information from the \"bridge\" > network inside the `Networks` map instead, which contains the same > information. This field was deprecated in Docker 1.9 and is scheduled > to be removed in Docker 17.12.0
    #[serde(rename = "MacAddress", skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Information about all networks that the container is connected to.
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<std::collections::HashMap<String, models::EndpointSettings>>,
}
impl NetworkSettings {
    /// NetworkSettings exposes the network settings in the API
    pub fn new() -> NetworkSettings {
        NetworkSettings {
            bridge: None,
            sandbox_id: None,
            hairpin_mode: None,
            link_local_ipv6_address: None,
            link_local_ipv6_prefix_len: None,
            ports: None,
            sandbox_key: None,
            secondary_ip_addresses: None,
            secondary_ipv6_addresses: None,
            endpoint_id: None,
            gateway: None,
            global_ipv6_address: None,
            global_ipv6_prefix_len: None,
            ip_address: None,
            ip_prefix_len: None,
            ipv6_gateway: None,
            mac_address: None,
            networks: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// NetworkingConfig : NetworkingConfig represents the container's networking configuration for each of its interfaces. It is used for the networking configs specified in the `docker create` and `docker network connect` commands.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkingConfig {
    /// A mapping of network name to endpoint configuration for that network. The endpoint configuration can be left empty to connect to that network with no particular endpoint configuration.
    #[serde(rename = "EndpointsConfig", skip_serializing_if = "Option::is_none")]
    pub endpoints_config: Option<
        std::collections::HashMap<String, models::EndpointSettings>,
    >,
}
impl NetworkingConfig {
    /// NetworkingConfig represents the container's networking configuration for each of its interfaces. It is used for the networking configs specified in the `docker create` and `docker network connect` commands.
    pub fn new() -> NetworkingConfig {
        NetworkingConfig {
            endpoints_config: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    /// Date and time at which the node was added to the swarm in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the node was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::NodeSpec>>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<models::NodeDescription>>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::NodeStatus>>,
    #[serde(
        rename = "ManagerStatus",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub manager_status: Option<Option<Box<models::ManagerStatus>>>,
}
impl Node {
    pub fn new() -> Node {
        Node {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            spec: None,
            description: None,
            status: None,
            manager_status: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// NodeDescription : NodeDescription encapsulates the properties of the Node as reported by the agent.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeDescription {
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(rename = "Platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<models::Platform>>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<models::ResourceObject>>,
    #[serde(rename = "Engine", skip_serializing_if = "Option::is_none")]
    pub engine: Option<Box<models::EngineDescription>>,
    #[serde(rename = "TLSInfo", skip_serializing_if = "Option::is_none")]
    pub tls_info: Option<Box<models::TlsInfo>>,
}
impl NodeDescription {
    /// NodeDescription encapsulates the properties of the Node as reported by the agent.
    pub fn new() -> NodeDescription {
        NodeDescription {
            hostname: None,
            platform: None,
            resources: None,
            engine: None,
            tls_info: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeSpec {
    /// Name for the node.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Role of the node.
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    /// Availability of the node.
    #[serde(rename = "Availability", skip_serializing_if = "Option::is_none")]
    pub availability: Option<Availability>,
}
impl NodeSpec {
    pub fn new() -> NodeSpec {
        NodeSpec {
            name: None,
            labels: None,
            role: None,
            availability: None,
        }
    }
}
/// Role of the node.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Role {
    #[serde(rename = "worker")]
    Worker,
    #[serde(rename = "manager")]
    Manager,
}
impl Default for Role {
    fn default() -> Role {
        Self::Worker
    }
}
/// Availability of the node.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Availability {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "drain")]
    Drain,
}
impl Default for Availability {
    fn default() -> Availability {
        Self::Active
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// NodeState : NodeState represents the state of a node.
/// NodeState represents the state of a node.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum NodeState {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "disconnected")]
    Disconnected,
}
impl std::fmt::Display for NodeState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Down => write!(f, "down"),
            Self::Ready => write!(f, "ready"),
            Self::Disconnected => write!(f, "disconnected"),
        }
    }
}
impl Default for NodeState {
    fn default() -> NodeState {
        Self::Unknown
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// NodeStatus : NodeStatus represents the status of a node.  It provides the current status of the node, as seen by the manager.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeStatus {
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<models::NodeState>,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// IP address of the node.
    #[serde(rename = "Addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
impl NodeStatus {
    /// NodeStatus represents the status of a node.  It provides the current status of the node, as seen by the manager.
    pub fn new() -> NodeStatus {
        NodeStatus {
            state: None,
            message: None,
            addr: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ObjectVersion : The version number of the object such as node, service, etc. This is needed to avoid conflicting writes. The client must send the version number along with the modified specification when updating these objects.  This approach ensures safe concurrency and determinism in that the change on the object may not be applied if the version number has changed from the last read. In other words, if two update requests specify the same base version, only one of the requests can succeed. As a result, two separate update requests that happen at the same time will not unintentionally overwrite each other.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectVersion {
    #[serde(rename = "Index", skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
}
impl ObjectVersion {
    /// The version number of the object such as node, service, etc. This is needed to avoid conflicting writes. The client must send the version number along with the modified specification when updating these objects.  This approach ensures safe concurrency and determinism in that the change on the object may not be applied if the version number has changed from the last read. In other words, if two update requests specify the same base version, only one of the requests can succeed. As a result, two separate update requests that happen at the same time will not unintentionally overwrite each other.
    pub fn new() -> ObjectVersion {
        ObjectVersion { index: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// OciDescriptor : A descriptor struct containing digest, media type, and size, as defined in the [OCI Content Descriptors Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OciDescriptor {
    /// The media type of the object this schema refers to.
    #[serde(rename = "mediaType", skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// The digest of the targeted content.
    #[serde(rename = "digest", skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    /// The size in bytes of the blob.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
impl OciDescriptor {
    /// A descriptor struct containing digest, media type, and size, as defined in the [OCI Content Descriptors Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/descriptor.md).
    pub fn new() -> OciDescriptor {
        OciDescriptor {
            media_type: None,
            digest: None,
            size: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// OciPlatform : Describes the platform which the image in the manifest runs on, as defined in the [OCI Image Index Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/image-index.md).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OciPlatform {
    /// The CPU architecture, for example `amd64` or `ppc64`.
    #[serde(rename = "architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The operating system, for example `linux` or `windows`.
    #[serde(rename = "os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// Optional field specifying the operating system version, for example on Windows `10.0.19041.1165`.
    #[serde(rename = "os.version", skip_serializing_if = "Option::is_none")]
    pub os_period_version: Option<String>,
    /// Optional field specifying an array of strings, each listing a required OS feature (for example on Windows `win32k`).
    #[serde(rename = "os.features", skip_serializing_if = "Option::is_none")]
    pub os_period_features: Option<Vec<String>>,
    /// Optional field specifying a variant of the CPU, for example `v7` to specify ARMv7 when architecture is `arm`.
    #[serde(rename = "variant", skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}
impl OciPlatform {
    /// Describes the platform which the image in the manifest runs on, as defined in the [OCI Image Index Specification](https://github.com/opencontainers/image-spec/blob/v1.0.1/image-index.md).
    pub fn new() -> OciPlatform {
        OciPlatform {
            architecture: None,
            os: None,
            os_period_version: None,
            os_period_features: None,
            variant: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PeerInfo : PeerInfo represents one peer of an overlay network.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeerInfo {
    /// ID of the peer-node in the Swarm cluster.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// IP-address of the peer-node in the Swarm cluster.
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}
impl PeerInfo {
    /// PeerInfo represents one peer of an overlay network.
    pub fn new() -> PeerInfo {
        PeerInfo { name: None, ip: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PeerNode : Represents a peer-node in the swarm
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PeerNode {
    /// Unique identifier of for this node in the swarm.
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// IP address and ports at which this node can be reached.
    #[serde(rename = "Addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
impl PeerNode {
    /// Represents a peer-node in the swarm
    pub fn new() -> PeerNode {
        PeerNode {
            node_id: None,
            addr: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Platform : Platform represents the platform (Arch/OS).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Platform {
    /// Architecture represents the hardware architecture (for example, `x86_64`).
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// OS represents the Operating System (for example, `linux` or `windows`).
    #[serde(rename = "OS", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}
impl Platform {
    /// Platform represents the platform (Arch/OS).
    pub fn new() -> Platform {
        Platform {
            architecture: None,
            os: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Plugin : A plugin for the Engine API
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Plugin {
    #[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    /// True if the plugin is running. False if the plugin is not running, only installed.
    #[serde(rename = "Enabled")]
    pub enabled: bool,
    #[serde(rename = "Settings")]
    pub settings: Box<models::PluginSettings>,
    /// plugin remote reference used to push/pull the plugin
    #[serde(rename = "PluginReference", skip_serializing_if = "Option::is_none")]
    pub plugin_reference: Option<String>,
    #[serde(rename = "Config")]
    pub config: Box<models::PluginConfig>,
}
impl Plugin {
    /// A plugin for the Engine API
    pub fn new(
        name: String,
        enabled: bool,
        settings: models::PluginSettings,
        config: models::PluginConfig,
    ) -> Plugin {
        Plugin {
            id: None,
            name,
            enabled,
            settings: Box::new(settings),
            plugin_reference: None,
            config: Box::new(config),
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PluginConfig : The config of a plugin.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfig {
    /// Docker Version used to create the plugin
    #[serde(rename = "DockerVersion", skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Documentation")]
    pub documentation: String,
    #[serde(rename = "Interface")]
    pub interface: Box<models::PluginConfigInterface>,
    #[serde(rename = "Entrypoint")]
    pub entrypoint: Vec<String>,
    #[serde(rename = "WorkDir")]
    pub work_dir: String,
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<models::PluginConfigUser>>,
    #[serde(rename = "Network")]
    pub network: Box<models::PluginConfigNetwork>,
    #[serde(rename = "Linux")]
    pub linux: Box<models::PluginConfigLinux>,
    #[serde(rename = "PropagatedMount")]
    pub propagated_mount: String,
    #[serde(rename = "IpcHost")]
    pub ipc_host: bool,
    #[serde(rename = "PidHost")]
    pub pid_host: bool,
    #[serde(rename = "Mounts")]
    pub mounts: Vec<models::PluginMount>,
    #[serde(rename = "Env")]
    pub env: Vec<models::PluginEnv>,
    #[serde(rename = "Args")]
    pub args: Box<models::PluginConfigArgs>,
    #[serde(rename = "rootfs", skip_serializing_if = "Option::is_none")]
    pub rootfs: Option<Box<models::PluginConfigRootfs>>,
}
impl PluginConfig {
    /// The config of a plugin.
    pub fn new(
        description: String,
        documentation: String,
        interface: models::PluginConfigInterface,
        entrypoint: Vec<String>,
        work_dir: String,
        network: models::PluginConfigNetwork,
        linux: models::PluginConfigLinux,
        propagated_mount: String,
        ipc_host: bool,
        pid_host: bool,
        mounts: Vec<models::PluginMount>,
        env: Vec<models::PluginEnv>,
        args: models::PluginConfigArgs,
    ) -> PluginConfig {
        PluginConfig {
            docker_version: None,
            description,
            documentation,
            interface: Box::new(interface),
            entrypoint,
            work_dir,
            user: None,
            network: Box::new(network),
            linux: Box::new(linux),
            propagated_mount,
            ipc_host,
            pid_host,
            mounts,
            env,
            args: Box::new(args),
            rootfs: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigArgs {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    pub value: Vec<String>,
}
impl PluginConfigArgs {
    pub fn new(
        name: String,
        description: String,
        settable: Vec<String>,
        value: Vec<String>,
    ) -> PluginConfigArgs {
        PluginConfigArgs {
            name,
            description,
            settable,
            value,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PluginConfigInterface : The interface between Docker and the plugin
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigInterface {
    #[serde(rename = "Types")]
    pub types: Vec<models::PluginInterfaceType>,
    #[serde(rename = "Socket")]
    pub socket: String,
    /// Protocol to use for clients connecting to the plugin.
    #[serde(rename = "ProtocolScheme", skip_serializing_if = "Option::is_none")]
    pub protocol_scheme: Option<ProtocolScheme>,
}
impl PluginConfigInterface {
    /// The interface between Docker and the plugin
    pub fn new(
        types: Vec<models::PluginInterfaceType>,
        socket: String,
    ) -> PluginConfigInterface {
        PluginConfigInterface {
            types,
            socket,
            protocol_scheme: None,
        }
    }
}
/// Protocol to use for clients connecting to the plugin.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum ProtocolScheme {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "moby.plugins.http/v1")]
    MobyPeriodPluginsPeriodHttpSlashV1,
}
impl Default for ProtocolScheme {
    fn default() -> ProtocolScheme {
        Self::Empty
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigLinux {
    #[serde(rename = "Capabilities")]
    pub capabilities: Vec<String>,
    #[serde(rename = "AllowAllDevices")]
    pub allow_all_devices: bool,
    #[serde(rename = "Devices")]
    pub devices: Vec<models::PluginDevice>,
}
impl PluginConfigLinux {
    pub fn new(
        capabilities: Vec<String>,
        allow_all_devices: bool,
        devices: Vec<models::PluginDevice>,
    ) -> PluginConfigLinux {
        PluginConfigLinux {
            capabilities,
            allow_all_devices,
            devices,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigNetwork {
    #[serde(rename = "Type")]
    pub r#type: String,
}
impl PluginConfigNetwork {
    pub fn new(r#type: String) -> PluginConfigNetwork {
        PluginConfigNetwork { r#type }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigRootfs {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "diff_ids", skip_serializing_if = "Option::is_none")]
    pub diff_ids: Option<Vec<String>>,
}
impl PluginConfigRootfs {
    pub fn new() -> PluginConfigRootfs {
        PluginConfigRootfs {
            r#type: None,
            diff_ids: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginConfigUser {
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<i32>,
    #[serde(rename = "GID", skip_serializing_if = "Option::is_none")]
    pub gid: Option<i32>,
}
impl PluginConfigUser {
    pub fn new() -> PluginConfigUser {
        PluginConfigUser {
            uid: None,
            gid: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginDevice {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,
    #[serde(rename = "Path")]
    pub path: String,
}
impl PluginDevice {
    pub fn new(
        name: String,
        description: String,
        settable: Vec<String>,
        path: String,
    ) -> PluginDevice {
        PluginDevice {
            name,
            description,
            settable,
            path,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginEnv {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,
    #[serde(rename = "Value")]
    pub value: String,
}
impl PluginEnv {
    pub fn new(
        name: String,
        description: String,
        settable: Vec<String>,
        value: String,
    ) -> PluginEnv {
        PluginEnv {
            name,
            description,
            settable,
            value,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginInterfaceType {
    #[serde(rename = "Prefix")]
    pub prefix: String,
    #[serde(rename = "Capability")]
    pub capability: String,
    #[serde(rename = "Version")]
    pub version: String,
}
impl PluginInterfaceType {
    pub fn new(
        prefix: String,
        capability: String,
        version: String,
    ) -> PluginInterfaceType {
        PluginInterfaceType {
            prefix,
            capability,
            version,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginMount {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Settable")]
    pub settable: Vec<String>,
    #[serde(rename = "Source")]
    pub source: String,
    #[serde(rename = "Destination")]
    pub destination: String,
    #[serde(rename = "Type")]
    pub r#type: String,
    #[serde(rename = "Options")]
    pub options: Vec<String>,
}
impl PluginMount {
    pub fn new(
        name: String,
        description: String,
        settable: Vec<String>,
        source: String,
        destination: String,
        r#type: String,
        options: Vec<String>,
    ) -> PluginMount {
        PluginMount {
            name,
            description,
            settable,
            source,
            destination,
            r#type,
            options,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PluginPrivilege : Describes a permission the user has to accept upon installing the plugin.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginPrivilege {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}
impl PluginPrivilege {
    /// Describes a permission the user has to accept upon installing the plugin.
    pub fn new() -> PluginPrivilege {
        PluginPrivilege {
            name: None,
            description: None,
            value: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PluginSettings : Settings that can be modified by users.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginSettings {
    #[serde(rename = "Mounts")]
    pub mounts: Vec<models::PluginMount>,
    #[serde(rename = "Env")]
    pub env: Vec<String>,
    #[serde(rename = "Args")]
    pub args: Vec<String>,
    #[serde(rename = "Devices")]
    pub devices: Vec<models::PluginDevice>,
}
impl PluginSettings {
    /// Settings that can be modified by users.
    pub fn new(
        mounts: Vec<models::PluginMount>,
        env: Vec<String>,
        args: Vec<String>,
        devices: Vec<models::PluginDevice>,
    ) -> PluginSettings {
        PluginSettings {
            mounts,
            env,
            args,
            devices,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PluginsInfo : Available plugins per type.  <p><br /></p>  > **Note**: Only unmanaged (V1) plugins are included in this list. > V1 plugins are \"lazily\" loaded, and are not returned in this list > if there is no resource using the plugin.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PluginsInfo {
    /// Names of available volume-drivers, and network-driver plugins.
    #[serde(rename = "Volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<Vec<String>>,
    /// Names of available network-drivers, and network-driver plugins.
    #[serde(rename = "Network", skip_serializing_if = "Option::is_none")]
    pub network: Option<Vec<String>>,
    /// Names of available authorization plugins.
    #[serde(rename = "Authorization", skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Vec<String>>,
    /// Names of available logging-drivers, and logging-driver plugins.
    #[serde(rename = "Log", skip_serializing_if = "Option::is_none")]
    pub log: Option<Vec<String>>,
}
impl PluginsInfo {
    /// Available plugins per type.  <p><br /></p>  > **Note**: Only unmanaged (V1) plugins are included in this list. > V1 plugins are \"lazily\" loaded, and are not returned in this list > if there is no resource using the plugin.
    pub fn new() -> PluginsInfo {
        PluginsInfo {
            volume: None,
            network: None,
            authorization: None,
            log: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Port : An open port on a container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Port {
    /// Host IP address that the container's port is mapped to
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// Port on the container
    #[serde(rename = "PrivatePort")]
    pub private_port: i32,
    /// Port exposed on the host
    #[serde(rename = "PublicPort", skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[serde(rename = "Type")]
    pub r#type: Type,
}
impl Port {
    /// An open port on a container
    pub fn new(private_port: i32, r#type: Type) -> Port {
        Port {
            ip: None,
            private_port,
            public_port: None,
            r#type,
        }
    }
}
///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Type {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "udp")]
    Udp,
    #[serde(rename = "sctp")]
    Sctp,
}
impl Default for Type {
    fn default() -> Type {
        Self::Tcp
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PortBinding : PortBinding represents a binding between a host IP address and a host port.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortBinding {
    /// Host IP address that the container's port is mapped to.
    #[serde(rename = "HostIp", skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,
    /// Host port number that the container's port is mapped to.
    #[serde(rename = "HostPort", skip_serializing_if = "Option::is_none")]
    pub host_port: Option<String>,
}
impl PortBinding {
    /// PortBinding represents a binding between a host IP address and a host port.
    pub fn new() -> PortBinding {
        PortBinding {
            host_ip: None,
            host_port: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// PortStatus : represents the port status of a task's host ports whose service has published host ports
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PortStatus {
    #[serde(rename = "Ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<models::EndpointPortConfig>>,
}
impl PortStatus {
    /// represents the port status of a task's host ports whose service has published host ports
    pub fn new() -> PortStatus {
        PortStatus { ports: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProcessConfig {
    #[serde(rename = "privileged", skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "tty", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    #[serde(rename = "entrypoint", skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<String>,
    #[serde(rename = "arguments", skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
}
impl ProcessConfig {
    pub fn new() -> ProcessConfig {
        ProcessConfig {
            privileged: None,
            user: None,
            tty: None,
            entrypoint: None,
            arguments: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProgressDetail {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    pub current: Option<i32>,
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}
impl ProgressDetail {
    pub fn new() -> ProgressDetail {
        ProgressDetail {
            current: None,
            total: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PushImageInfo {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "progress", skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    #[serde(rename = "progressDetail", skip_serializing_if = "Option::is_none")]
    pub progress_detail: Option<Box<models::ProgressDetail>>,
}
impl PushImageInfo {
    pub fn new() -> PushImageInfo {
        PushImageInfo {
            error: None,
            status: None,
            progress: None,
            progress_detail: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Reachability : Reachability represents the reachability of a node.
/// Reachability represents the reachability of a node.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Reachability {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "unreachable")]
    Unreachable,
    #[serde(rename = "reachable")]
    Reachable,
}
impl std::fmt::Display for Reachability {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "unknown"),
            Self::Unreachable => write!(f, "unreachable"),
            Self::Reachable => write!(f, "reachable"),
        }
    }
}
impl Default for Reachability {
    fn default() -> Reachability {
        Self::Unknown
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// RegistryServiceConfig : RegistryServiceConfig stores daemon registry services configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryServiceConfig {
    /// List of IP ranges to which nondistributable artifacts can be pushed, using the CIDR syntax [RFC 4632](https://tools.ietf.org/html/4632).  Some images (for example, Windows base images) contain artifacts whose distribution is restricted by license. When these images are pushed to a registry, restricted artifacts are not included.  This configuration override this behavior, and enables the daemon to push nondistributable artifacts to all registries whose resolved IP address is within the subnet described by the CIDR syntax.  This option is useful when pushing images containing nondistributable artifacts to a registry on an air-gapped network so hosts on that network can pull the images without connecting to another server.  > **Warning**: Nondistributable artifacts typically have restrictions > on how and where they can be distributed and shared. Only use this > feature to push artifacts to private registries and ensure that you > are in compliance with any terms that cover redistributing > nondistributable artifacts.
    #[serde(
        rename = "AllowNondistributableArtifactsCIDRs",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_nondistributable_artifacts_cidrs: Option<Vec<String>>,
    /// List of registry hostnames to which nondistributable artifacts can be pushed, using the format `<hostname>[:<port>]` or `<IP address>[:<port>]`.  Some images (for example, Windows base images) contain artifacts whose distribution is restricted by license. When these images are pushed to a registry, restricted artifacts are not included.  This configuration override this behavior for the specified registries.  This option is useful when pushing images containing nondistributable artifacts to a registry on an air-gapped network so hosts on that network can pull the images without connecting to another server.  > **Warning**: Nondistributable artifacts typically have restrictions > on how and where they can be distributed and shared. Only use this > feature to push artifacts to private registries and ensure that you > are in compliance with any terms that cover redistributing > nondistributable artifacts.
    #[serde(
        rename = "AllowNondistributableArtifactsHostnames",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_nondistributable_artifacts_hostnames: Option<Vec<String>>,
    /// List of IP ranges of insecure registries, using the CIDR syntax ([RFC 4632](https://tools.ietf.org/html/4632)). Insecure registries accept un-encrypted (HTTP) and/or untrusted (HTTPS with certificates from unknown CAs) communication.  By default, local registries (`127.0.0.0/8`) are configured as insecure. All other registries are secure. Communicating with an insecure registry is not possible if the daemon assumes that registry is secure.  This configuration override this behavior, insecure communication with registries whose resolved IP address is within the subnet described by the CIDR syntax.  Registries can also be marked insecure by hostname. Those registries are listed under `IndexConfigs` and have their `Secure` field set to `false`.  > **Warning**: Using this option can be useful when running a local > registry, but introduces security vulnerabilities. This option > should therefore ONLY be used for testing purposes. For increased > security, users should add their CA to their system's list of trusted > CAs instead of enabling this option.
    #[serde(rename = "InsecureRegistryCIDRs", skip_serializing_if = "Option::is_none")]
    pub insecure_registry_cidrs: Option<Vec<String>>,
    #[serde(rename = "IndexConfigs", skip_serializing_if = "Option::is_none")]
    pub index_configs: Option<std::collections::HashMap<String, models::IndexInfo>>,
    /// List of registry URLs that act as a mirror for the official (`docker.io`) registry.
    #[serde(rename = "Mirrors", skip_serializing_if = "Option::is_none")]
    pub mirrors: Option<Vec<String>>,
}
impl RegistryServiceConfig {
    /// RegistryServiceConfig stores daemon registry services configuration.
    pub fn new() -> RegistryServiceConfig {
        RegistryServiceConfig {
            allow_nondistributable_artifacts_cidrs: None,
            allow_nondistributable_artifacts_hostnames: None,
            insecure_registry_cidrs: None,
            index_configs: None,
            mirrors: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ResourceObject : An object describing the resources which can be advertised by a node and requested by a task.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceObject {
    #[serde(rename = "NanoCPUs", skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    #[serde(rename = "MemoryBytes", skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<i64>,
    /// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, `GPU=UUID1`).
    #[serde(rename = "GenericResources", skip_serializing_if = "Option::is_none")]
    pub generic_resources: Option<Vec<models::GenericResourcesInner>>,
}
impl ResourceObject {
    /// An object describing the resources which can be advertised by a node and requested by a task.
    pub fn new() -> ResourceObject {
        ResourceObject {
            nano_cpus: None,
            memory_bytes: None,
            generic_resources: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Resources : A container's resources (cgroups config, ulimits, etc)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resources {
    /// An integer value representing this container's relative CPU weight versus other containers.
    #[serde(rename = "CpuShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<i32>,
    /// Memory limit in bytes.
    #[serde(rename = "Memory", skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// Path to `cgroups` under which the container's `cgroup` is created. If the path is not absolute, the path is considered to be relative to the `cgroups` path of the init process. Cgroups are created if they do not already exist.
    #[serde(rename = "CgroupParent", skip_serializing_if = "Option::is_none")]
    pub cgroup_parent: Option<String>,
    /// Block IO weight (relative weight).
    #[serde(rename = "BlkioWeight", skip_serializing_if = "Option::is_none")]
    pub blkio_weight: Option<u32>,
    /// Block IO weight (relative device weight) in the form:  ``` [{\"Path\": \"device_path\", \"Weight\": weight}] ```
    #[serde(rename = "BlkioWeightDevice", skip_serializing_if = "Option::is_none")]
    pub blkio_weight_device: Option<Vec<models::ResourcesBlkioWeightDeviceInner>>,
    /// Limit read rate (bytes per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_bps: Option<Vec<models::ThrottleDevice>>,
    /// Limit write rate (bytes per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteBps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_bps: Option<Vec<models::ThrottleDevice>>,
    /// Limit read rate (IO per second) from a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceReadIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_read_i_ops: Option<Vec<models::ThrottleDevice>>,
    /// Limit write rate (IO per second) to a device, in the form:  ``` [{\"Path\": \"device_path\", \"Rate\": rate}] ```
    #[serde(rename = "BlkioDeviceWriteIOps", skip_serializing_if = "Option::is_none")]
    pub blkio_device_write_i_ops: Option<Vec<models::ThrottleDevice>>,
    /// The length of a CPU period in microseconds.
    #[serde(rename = "CpuPeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_period: Option<i64>,
    /// Microseconds of CPU time that the container can get in a CPU period.
    #[serde(rename = "CpuQuota", skip_serializing_if = "Option::is_none")]
    pub cpu_quota: Option<i64>,
    /// The length of a CPU real-time period in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimePeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_period: Option<i64>,
    /// The length of a CPU real-time runtime in microseconds. Set to 0 to allocate no time allocated to real-time tasks.
    #[serde(rename = "CpuRealtimeRuntime", skip_serializing_if = "Option::is_none")]
    pub cpu_realtime_runtime: Option<i64>,
    /// CPUs in which to allow execution (e.g., `0-3`, `0,1`).
    #[serde(rename = "CpusetCpus", skip_serializing_if = "Option::is_none")]
    pub cpuset_cpus: Option<String>,
    /// Memory nodes (MEMs) in which to allow execution (0-3, 0,1). Only effective on NUMA systems.
    #[serde(rename = "CpusetMems", skip_serializing_if = "Option::is_none")]
    pub cpuset_mems: Option<String>,
    /// A list of devices to add to the container.
    #[serde(rename = "Devices", skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<models::DeviceMapping>>,
    /// a list of cgroup rules to apply to the container
    #[serde(rename = "DeviceCgroupRules", skip_serializing_if = "Option::is_none")]
    pub device_cgroup_rules: Option<Vec<String>>,
    /// A list of requests for devices to be sent to device drivers.
    #[serde(rename = "DeviceRequests", skip_serializing_if = "Option::is_none")]
    pub device_requests: Option<Vec<models::DeviceRequest>>,
    /// Hard limit for kernel TCP buffer memory (in bytes). Depending on the OCI runtime in use, this option may be ignored. It is no longer supported by the default (runc) runtime.  This field is omitted when empty.
    #[serde(rename = "KernelMemoryTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<i64>,
    /// Memory soft limit in bytes.
    #[serde(rename = "MemoryReservation", skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// Total memory limit (memory + swap). Set as `-1` to enable unlimited swap.
    #[serde(rename = "MemorySwap", skip_serializing_if = "Option::is_none")]
    pub memory_swap: Option<i64>,
    /// Tune a container's memory swappiness behavior. Accepts an integer between 0 and 100.
    #[serde(rename = "MemorySwappiness", skip_serializing_if = "Option::is_none")]
    pub memory_swappiness: Option<u64>,
    /// CPU quota in units of 10<sup>-9</sup> CPUs.
    #[serde(rename = "NanoCpus", skip_serializing_if = "Option::is_none")]
    pub nano_cpus: Option<i64>,
    /// Disable OOM Killer for the container.
    #[serde(rename = "OomKillDisable", skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(
        rename = "Init",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub init: Option<Option<bool>>,
    /// Tune a container's PIDs limit. Set `0` or `-1` for unlimited, or `null` to not change.
    #[serde(
        rename = "PidsLimit",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub pids_limit: Option<Option<i64>>,
    /// A list of resource limits to set in the container. For example:  ``` {\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048} ```
    #[serde(rename = "Ulimits", skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<models::ResourcesUlimitsInner>>,
    /// The number of usable CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuCount", skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// The usable percentage of the available CPUs (Windows only).  On Windows Server containers, the processor resource controls are mutually exclusive. The order of precedence is `CPUCount` first, then `CPUShares`, and `CPUPercent` last.
    #[serde(rename = "CpuPercent", skip_serializing_if = "Option::is_none")]
    pub cpu_percent: Option<i64>,
    /// Maximum IOps for the container system drive (Windows only)
    #[serde(rename = "IOMaximumIOps", skip_serializing_if = "Option::is_none")]
    pub io_maximum_i_ops: Option<i64>,
    /// Maximum IO in bytes per second for the container system drive (Windows only).
    #[serde(rename = "IOMaximumBandwidth", skip_serializing_if = "Option::is_none")]
    pub io_maximum_bandwidth: Option<i64>,
}
impl Resources {
    /// A container's resources (cgroups config, ulimits, etc)
    pub fn new() -> Resources {
        Resources {
            cpu_shares: None,
            memory: None,
            cgroup_parent: None,
            blkio_weight: None,
            blkio_weight_device: None,
            blkio_device_read_bps: None,
            blkio_device_write_bps: None,
            blkio_device_read_i_ops: None,
            blkio_device_write_i_ops: None,
            cpu_period: None,
            cpu_quota: None,
            cpu_realtime_period: None,
            cpu_realtime_runtime: None,
            cpuset_cpus: None,
            cpuset_mems: None,
            devices: None,
            device_cgroup_rules: None,
            device_requests: None,
            kernel_memory_tcp: None,
            memory_reservation: None,
            memory_swap: None,
            memory_swappiness: None,
            nano_cpus: None,
            oom_kill_disable: None,
            init: None,
            pids_limit: None,
            ulimits: None,
            cpu_count: None,
            cpu_percent: None,
            io_maximum_i_ops: None,
            io_maximum_bandwidth: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesBlkioWeightDeviceInner {
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}
impl ResourcesBlkioWeightDeviceInner {
    pub fn new() -> ResourcesBlkioWeightDeviceInner {
        ResourcesBlkioWeightDeviceInner {
            path: None,
            weight: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourcesUlimitsInner {
    /// Name of ulimit
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Soft limit
    #[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
    pub soft: Option<i32>,
    /// Hard limit
    #[serde(rename = "Hard", skip_serializing_if = "Option::is_none")]
    pub hard: Option<i32>,
}
impl ResourcesUlimitsInner {
    pub fn new() -> ResourcesUlimitsInner {
        ResourcesUlimitsInner {
            name: None,
            soft: None,
            hard: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// RestartPolicy : The behavior to apply when the container exits. The default is not to restart.  An ever increasing delay (double the previous delay, starting at 100ms) is added before each restart to prevent flooding the server.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestartPolicy {
    /// - Empty string means not to restart - `no` Do not automatically restart - `always` Always restart - `unless-stopped` Restart always except when the user has manually stopped the container - `on-failure` Restart only when the container exit code is non-zero
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    /// If `on-failure` is used, the number of times to retry before giving up.
    #[serde(rename = "MaximumRetryCount", skip_serializing_if = "Option::is_none")]
    pub maximum_retry_count: Option<i32>,
}
impl RestartPolicy {
    /// The behavior to apply when the container exits. The default is not to restart.  An ever increasing delay (double the previous delay, starting at 100ms) is added before each restart to prevent flooding the server.
    pub fn new() -> RestartPolicy {
        RestartPolicy {
            name: None,
            maximum_retry_count: None,
        }
    }
}
/// - Empty string means not to restart - `no` Do not automatically restart - `always` Always restart - `unless-stopped` Restart always except when the user has manually stopped the container - `on-failure` Restart only when the container exit code is non-zero
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Name {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "no")]
    No,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "unless-stopped")]
    UnlessStopped,
    #[serde(rename = "on-failure")]
    OnFailure,
}
impl Default for Name {
    fn default() -> Name {
        Self::Empty
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// Runtime : Runtime describes an [OCI compliant](https://github.com/opencontainers/runtime-spec) runtime.  The runtime is invoked by the daemon via the `containerd` daemon. OCI runtimes act as an interface to the Linux kernel namespaces, cgroups, and SELinux.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Runtime {
    /// Name and, optional, path, of the OCI executable binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result.
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// List of command-line arguments to pass to the runtime when invoked.
    #[serde(
        rename = "runtimeArgs",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub runtime_args: Option<Option<Vec<String>>>,
    /// Information specific to the runtime.  While this API specification does not define data provided by runtimes, the following well-known properties may be provided by runtimes:  `org.opencontainers.runtime-spec.features`: features structure as defined in the [OCI Runtime Specification](https://github.com/opencontainers/runtime-spec/blob/main/features.md), in a JSON string representation.  <p><br /></p>  > **Note**: The information returned in this field, including the > formatting of values and labels, should not be considered stable, > and may change without notice.
    #[serde(
        rename = "status",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub status: Option<Option<std::collections::HashMap<String, String>>>,
}
impl Runtime {
    /// Runtime describes an [OCI compliant](https://github.com/opencontainers/runtime-spec) runtime.  The runtime is invoked by the daemon via the `containerd` daemon. OCI runtimes act as an interface to the Linux kernel namespaces, cgroups, and SELinux.
    pub fn new() -> Runtime {
        Runtime {
            path: None,
            runtime_args: None,
            status: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::SecretSpec>>,
}
impl Secret {
    pub fn new() -> Secret {
        Secret {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretCreateRequest {
    /// User-defined name of the secret.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) data to store as secret.  This field is only used to _create_ a secret, and is not returned by other endpoints.
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<Box<models::Driver>>,
    #[serde(rename = "Templating", skip_serializing_if = "Option::is_none")]
    pub templating: Option<Box<models::Driver>>,
}
impl SecretCreateRequest {
    pub fn new() -> SecretCreateRequest {
        SecretCreateRequest {
            name: None,
            labels: None,
            data: None,
            driver: None,
            templating: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecretSpec {
    /// User-defined name of the secret.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// Base64-url-safe-encoded ([RFC 4648](https://tools.ietf.org/html/rfc4648#section-5)) data to store as secret.  This field is only used to _create_ a secret, and is not returned by other endpoints.
    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<Box<models::Driver>>,
    #[serde(rename = "Templating", skip_serializing_if = "Option::is_none")]
    pub templating: Option<Box<models::Driver>>,
}
impl SecretSpec {
    pub fn new() -> SecretSpec {
        SecretSpec {
            name: None,
            labels: None,
            data: None,
            driver: None,
            templating: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::ServiceSpec>>,
    #[serde(rename = "Endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Box<models::ServiceEndpoint>>,
    #[serde(rename = "UpdateStatus", skip_serializing_if = "Option::is_none")]
    pub update_status: Option<Box<models::ServiceUpdateStatus>>,
    #[serde(rename = "ServiceStatus", skip_serializing_if = "Option::is_none")]
    pub service_status: Option<Box<models::ServiceServiceStatus>>,
    #[serde(rename = "JobStatus", skip_serializing_if = "Option::is_none")]
    pub job_status: Option<Box<models::ServiceJobStatus>>,
}
impl Service {
    pub fn new() -> Service {
        Service {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            spec: None,
            endpoint: None,
            update_status: None,
            service_status: None,
            job_status: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCreateRequest {
    /// Name of the service.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TaskTemplate", skip_serializing_if = "Option::is_none")]
    pub task_template: Option<Box<models::TaskSpec>>,
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<models::ServiceSpecMode>>,
    #[serde(rename = "UpdateConfig", skip_serializing_if = "Option::is_none")]
    pub update_config: Option<Box<models::ServiceSpecUpdateConfig>>,
    #[serde(rename = "RollbackConfig", skip_serializing_if = "Option::is_none")]
    pub rollback_config: Option<Box<models::ServiceSpecRollbackConfig>>,
    /// Specifies which networks the service should attach to.  Deprecated: This field is deprecated since v1.44. The Networks field in TaskSpec should be used instead.
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<models::NetworkAttachmentConfig>>,
    #[serde(rename = "EndpointSpec", skip_serializing_if = "Option::is_none")]
    pub endpoint_spec: Option<Box<models::EndpointSpec>>,
}
impl ServiceCreateRequest {
    pub fn new() -> ServiceCreateRequest {
        ServiceCreateRequest {
            name: None,
            labels: None,
            task_template: None,
            mode: None,
            update_config: None,
            rollback_config: None,
            networks: None,
            endpoint_spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceCreateResponse : contains the information returned to a client on the creation of a new service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceCreateResponse {
    /// The ID of the created service.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Optional warning message.  FIXME(thaJeztah): this should have \"omitempty\" in the generated type.
    #[serde(
        rename = "Warnings",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub warnings: Option<Option<Vec<String>>>,
}
impl ServiceCreateResponse {
    /// contains the information returned to a client on the creation of a new service.
    pub fn new() -> ServiceCreateResponse {
        ServiceCreateResponse {
            id: None,
            warnings: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::EndpointSpec>>,
    #[serde(rename = "Ports", skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<models::EndpointPortConfig>>,
    #[serde(rename = "VirtualIPs", skip_serializing_if = "Option::is_none")]
    pub virtual_ips: Option<Vec<models::ServiceEndpointVirtualIpsInner>>,
}
impl ServiceEndpoint {
    pub fn new() -> ServiceEndpoint {
        ServiceEndpoint {
            spec: None,
            ports: None,
            virtual_ips: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceEndpointVirtualIpsInner {
    #[serde(rename = "NetworkID", skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    #[serde(rename = "Addr", skip_serializing_if = "Option::is_none")]
    pub addr: Option<String>,
}
impl ServiceEndpointVirtualIpsInner {
    pub fn new() -> ServiceEndpointVirtualIpsInner {
        ServiceEndpointVirtualIpsInner {
            network_id: None,
            addr: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceJobStatus : The status of the service when it is in one of ReplicatedJob or GlobalJob modes. Absent on Replicated and Global mode services. The JobIteration is an ObjectVersion, but unlike the Service's version, does not need to be sent with an update request.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceJobStatus {
    #[serde(rename = "JobIteration", skip_serializing_if = "Option::is_none")]
    pub job_iteration: Option<Box<models::ObjectVersion>>,
    /// The last time, as observed by the server, that this job was started.
    #[serde(rename = "LastExecution", skip_serializing_if = "Option::is_none")]
    pub last_execution: Option<String>,
}
impl ServiceJobStatus {
    /// The status of the service when it is in one of ReplicatedJob or GlobalJob modes. Absent on Replicated and Global mode services. The JobIteration is an ObjectVersion, but unlike the Service's version, does not need to be sent with an update request.
    pub fn new() -> ServiceJobStatus {
        ServiceJobStatus {
            job_iteration: None,
            last_execution: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceServiceStatus : The status of the service's tasks. Provided only when requested as part of a ServiceList operation.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceServiceStatus {
    /// The number of tasks for the service currently in the Running state.
    #[serde(rename = "RunningTasks", skip_serializing_if = "Option::is_none")]
    pub running_tasks: Option<i32>,
    /// The number of tasks for the service desired to be running. For replicated services, this is the replica count from the service spec. For global services, this is computed by taking count of all tasks for the service with a Desired State other than Shutdown.
    #[serde(rename = "DesiredTasks", skip_serializing_if = "Option::is_none")]
    pub desired_tasks: Option<i32>,
    /// The number of tasks for a job that are in the Completed state. This field must be cross-referenced with the service type, as the value of 0 may mean the service is not in a job mode, or it may mean the job-mode service has no tasks yet Completed.
    #[serde(rename = "CompletedTasks", skip_serializing_if = "Option::is_none")]
    pub completed_tasks: Option<i32>,
}
impl ServiceServiceStatus {
    /// The status of the service's tasks. Provided only when requested as part of a ServiceList operation.
    pub fn new() -> ServiceServiceStatus {
        ServiceServiceStatus {
            running_tasks: None,
            desired_tasks: None,
            completed_tasks: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceSpec : User modifiable configuration for a service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpec {
    /// Name of the service.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TaskTemplate", skip_serializing_if = "Option::is_none")]
    pub task_template: Option<Box<models::TaskSpec>>,
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<models::ServiceSpecMode>>,
    #[serde(rename = "UpdateConfig", skip_serializing_if = "Option::is_none")]
    pub update_config: Option<Box<models::ServiceSpecUpdateConfig>>,
    #[serde(rename = "RollbackConfig", skip_serializing_if = "Option::is_none")]
    pub rollback_config: Option<Box<models::ServiceSpecRollbackConfig>>,
    /// Specifies which networks the service should attach to.  Deprecated: This field is deprecated since v1.44. The Networks field in TaskSpec should be used instead.
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<models::NetworkAttachmentConfig>>,
    #[serde(rename = "EndpointSpec", skip_serializing_if = "Option::is_none")]
    pub endpoint_spec: Option<Box<models::EndpointSpec>>,
}
impl ServiceSpec {
    /// User modifiable configuration for a service.
    pub fn new() -> ServiceSpec {
        ServiceSpec {
            name: None,
            labels: None,
            task_template: None,
            mode: None,
            update_config: None,
            rollback_config: None,
            networks: None,
            endpoint_spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceSpecMode : Scheduling mode for the service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecMode {
    #[serde(rename = "Replicated", skip_serializing_if = "Option::is_none")]
    pub replicated: Option<Box<models::ServiceSpecModeReplicated>>,
    #[serde(rename = "Global", skip_serializing_if = "Option::is_none")]
    pub global: Option<serde_json::Value>,
    #[serde(rename = "ReplicatedJob", skip_serializing_if = "Option::is_none")]
    pub replicated_job: Option<Box<models::ServiceSpecModeReplicatedJob>>,
    /// The mode used for services which run a task to the completed state on each valid node.
    #[serde(rename = "GlobalJob", skip_serializing_if = "Option::is_none")]
    pub global_job: Option<serde_json::Value>,
}
impl ServiceSpecMode {
    /// Scheduling mode for the service.
    pub fn new() -> ServiceSpecMode {
        ServiceSpecMode {
            replicated: None,
            global: None,
            replicated_job: None,
            global_job: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecModeReplicated {
    #[serde(rename = "Replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
}
impl ServiceSpecModeReplicated {
    pub fn new() -> ServiceSpecModeReplicated {
        ServiceSpecModeReplicated {
            replicas: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceSpecModeReplicatedJob : The mode used for services with a finite number of tasks that run to a completed state.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecModeReplicatedJob {
    /// The maximum number of replicas to run simultaneously.
    #[serde(rename = "MaxConcurrent", skip_serializing_if = "Option::is_none")]
    pub max_concurrent: Option<i64>,
    /// The total number of replicas desired to reach the Completed state. If unset, will default to the value of `MaxConcurrent`
    #[serde(rename = "TotalCompletions", skip_serializing_if = "Option::is_none")]
    pub total_completions: Option<i64>,
}
impl ServiceSpecModeReplicatedJob {
    /// The mode used for services with a finite number of tasks that run to a completed state.
    pub fn new() -> ServiceSpecModeReplicatedJob {
        ServiceSpecModeReplicatedJob {
            max_concurrent: None,
            total_completions: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceSpecRollbackConfig : Specification for the rollback strategy of the service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecRollbackConfig {
    /// Maximum number of tasks to be rolled back in one iteration (0 means unlimited parallelism).
    #[serde(rename = "Parallelism", skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    /// Amount of time between rollback iterations, in nanoseconds.
    #[serde(rename = "Delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// Action to take if an rolled back task fails to run, or stops running during the rollback.
    #[serde(rename = "FailureAction", skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<FailureAction>,
    /// Amount of time to monitor each rolled back task for failures, in nanoseconds.
    #[serde(rename = "Monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: Option<i64>,
    /// The fraction of tasks that may fail during a rollback before the failure action is invoked, specified as a floating point number between 0 and 1.
    #[serde(rename = "MaxFailureRatio", skip_serializing_if = "Option::is_none")]
    pub max_failure_ratio: Option<f64>,
    /// The order of operations when rolling back a task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
    #[serde(rename = "Order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}
impl ServiceSpecRollbackConfig {
    /// Specification for the rollback strategy of the service.
    pub fn new() -> ServiceSpecRollbackConfig {
        ServiceSpecRollbackConfig {
            parallelism: None,
            delay: None,
            failure_action: None,
            monitor: None,
            max_failure_ratio: None,
            order: None,
        }
    }
}
/// Action to take if an rolled back task fails to run, or stops running during the rollback.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum FailureAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
}
impl Default for FailureAction {
    fn default() -> FailureAction {
        Self::Continue
    }
}
/// The order of operations when rolling back a task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Order {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}
impl Default for Order {
    fn default() -> Order {
        Self::StopFirst
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceSpecUpdateConfig : Specification for the update strategy of the service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceSpecUpdateConfig {
    /// Maximum number of tasks to be updated in one iteration (0 means unlimited parallelism).
    #[serde(rename = "Parallelism", skip_serializing_if = "Option::is_none")]
    pub parallelism: Option<i64>,
    /// Amount of time between updates, in nanoseconds.
    #[serde(rename = "Delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// Action to take if an updated task fails to run, or stops running during the update.
    #[serde(rename = "FailureAction", skip_serializing_if = "Option::is_none")]
    pub failure_action: Option<FailureAction>,
    /// Amount of time to monitor each updated task for failures, in nanoseconds.
    #[serde(rename = "Monitor", skip_serializing_if = "Option::is_none")]
    pub monitor: Option<i64>,
    /// The fraction of tasks that may fail during an update before the failure action is invoked, specified as a floating point number between 0 and 1.
    #[serde(rename = "MaxFailureRatio", skip_serializing_if = "Option::is_none")]
    pub max_failure_ratio: Option<f64>,
    /// The order of operations when rolling out an updated task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
    #[serde(rename = "Order", skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}
impl ServiceSpecUpdateConfig {
    /// Specification for the update strategy of the service.
    pub fn new() -> ServiceSpecUpdateConfig {
        ServiceSpecUpdateConfig {
            parallelism: None,
            delay: None,
            failure_action: None,
            monitor: None,
            max_failure_ratio: None,
            order: None,
        }
    }
}
/// Action to take if an updated task fails to run, or stops running during the update.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum FailureAction {
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "rollback")]
    Rollback,
}
impl Default for FailureAction {
    fn default() -> FailureAction {
        Self::Continue
    }
}
/// The order of operations when rolling out an updated task. Either the old task is shut down before the new task is started, or the new task is started before the old task is shut down.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Order {
    #[serde(rename = "stop-first")]
    StopFirst,
    #[serde(rename = "start-first")]
    StartFirst,
}
impl Default for Order {
    fn default() -> Order {
        Self::StopFirst
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpdateRequest {
    /// Name of the service.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TaskTemplate", skip_serializing_if = "Option::is_none")]
    pub task_template: Option<Box<models::TaskSpec>>,
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Box<models::ServiceSpecMode>>,
    #[serde(rename = "UpdateConfig", skip_serializing_if = "Option::is_none")]
    pub update_config: Option<Box<models::ServiceSpecUpdateConfig>>,
    #[serde(rename = "RollbackConfig", skip_serializing_if = "Option::is_none")]
    pub rollback_config: Option<Box<models::ServiceSpecRollbackConfig>>,
    /// Specifies which networks the service should attach to.  Deprecated: This field is deprecated since v1.44. The Networks field in TaskSpec should be used instead.
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<models::NetworkAttachmentConfig>>,
    #[serde(rename = "EndpointSpec", skip_serializing_if = "Option::is_none")]
    pub endpoint_spec: Option<Box<models::EndpointSpec>>,
}
impl ServiceUpdateRequest {
    pub fn new() -> ServiceUpdateRequest {
        ServiceUpdateRequest {
            name: None,
            labels: None,
            task_template: None,
            mode: None,
            update_config: None,
            rollback_config: None,
            networks: None,
            endpoint_spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpdateResponse {
    /// Optional warning messages
    #[serde(rename = "Warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
impl ServiceUpdateResponse {
    pub fn new() -> ServiceUpdateResponse {
        ServiceUpdateResponse {
            warnings: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// ServiceUpdateStatus : The status of a service update.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceUpdateStatus {
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[serde(rename = "StartedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "CompletedAt", skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
impl ServiceUpdateStatus {
    /// The status of a service update.
    pub fn new() -> ServiceUpdateStatus {
        ServiceUpdateStatus {
            state: None,
            started_at: None,
            completed_at: None,
            message: None,
        }
    }
}
///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum State {
    #[serde(rename = "updating")]
    Updating,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "completed")]
    Completed,
}
impl Default for State {
    fn default() -> State {
        Self::Updating
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Swarm {
    /// The ID of the swarm.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    /// Date and time at which the swarm was initialised in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Date and time at which the swarm was last updated in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::SwarmSpec>>,
    #[serde(rename = "TLSInfo", skip_serializing_if = "Option::is_none")]
    pub tls_info: Option<Box<models::TlsInfo>>,
    /// Whether there is currently a root CA rotation in progress for the swarm
    #[serde(rename = "RootRotationInProgress", skip_serializing_if = "Option::is_none")]
    pub root_rotation_in_progress: Option<bool>,
    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. If no port is set or is set to 0, the default port (4789) is used.
    #[serde(rename = "DataPathPort", skip_serializing_if = "Option::is_none")]
    pub data_path_port: Option<i32>,
    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[serde(rename = "DefaultAddrPool", skip_serializing_if = "Option::is_none")]
    pub default_addr_pool: Option<Vec<String>>,
    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[serde(rename = "SubnetSize", skip_serializing_if = "Option::is_none")]
    pub subnet_size: Option<i32>,
    #[serde(rename = "JoinTokens", skip_serializing_if = "Option::is_none")]
    pub join_tokens: Option<Box<models::JoinTokens>>,
}
impl Swarm {
    pub fn new() -> Swarm {
        Swarm {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            spec: None,
            tls_info: None,
            root_rotation_in_progress: None,
            data_path_port: None,
            default_addr_pool: None,
            subnet_size: None,
            join_tokens: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmInfo : Represents generic information about swarm.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmInfo {
    /// Unique identifier of for this node in the swarm.
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// IP address at which this node can be reached by other nodes in the swarm.
    #[serde(rename = "NodeAddr", skip_serializing_if = "Option::is_none")]
    pub node_addr: Option<String>,
    #[serde(rename = "LocalNodeState", skip_serializing_if = "Option::is_none")]
    pub local_node_state: Option<models::LocalNodeState>,
    #[serde(rename = "ControlAvailable", skip_serializing_if = "Option::is_none")]
    pub control_available: Option<bool>,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// List of ID's and addresses of other managers in the swarm.
    #[serde(
        rename = "RemoteManagers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_managers: Option<Option<Vec<models::PeerNode>>>,
    /// Total number of nodes in the swarm.
    #[serde(
        rename = "Nodes",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub nodes: Option<Option<i32>>,
    /// Total number of managers in the swarm.
    #[serde(
        rename = "Managers",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub managers: Option<Option<i32>>,
    #[serde(
        rename = "Cluster",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub cluster: Option<Option<Box<models::ClusterInfo>>>,
}
impl SwarmInfo {
    /// Represents generic information about swarm.
    pub fn new() -> SwarmInfo {
        SwarmInfo {
            node_id: None,
            node_addr: None,
            local_node_state: None,
            control_available: None,
            error: None,
            remote_managers: None,
            nodes: None,
            managers: None,
            cluster: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmInitRequest {
    /// Listen address used for inter-manager communication, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP). This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the default swarm listening port is used.
    #[serde(rename = "ListenAddr", skip_serializing_if = "Option::is_none")]
    pub listen_addr: Option<String>,
    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[serde(rename = "AdvertiseAddr", skip_serializing_if = "Option::is_none")]
    pub advertise_addr: Option<String>,
    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other  nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[serde(rename = "DataPathAddr", skip_serializing_if = "Option::is_none")]
    pub data_path_addr: Option<String>,
    /// DataPathPort specifies the data path port number for data traffic. Acceptable port range is 1024 to 49151. if no port is set or is set to 0, default port 4789 will be used.
    #[serde(rename = "DataPathPort", skip_serializing_if = "Option::is_none")]
    pub data_path_port: Option<i32>,
    /// Default Address Pool specifies default subnet pools for global scope networks.
    #[serde(rename = "DefaultAddrPool", skip_serializing_if = "Option::is_none")]
    pub default_addr_pool: Option<Vec<String>>,
    /// Force creation of a new swarm.
    #[serde(rename = "ForceNewCluster", skip_serializing_if = "Option::is_none")]
    pub force_new_cluster: Option<bool>,
    /// SubnetSize specifies the subnet size of the networks created from the default subnet pool.
    #[serde(rename = "SubnetSize", skip_serializing_if = "Option::is_none")]
    pub subnet_size: Option<i32>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::SwarmSpec>>,
}
impl SwarmInitRequest {
    pub fn new() -> SwarmInitRequest {
        SwarmInitRequest {
            listen_addr: None,
            advertise_addr: None,
            data_path_addr: None,
            data_path_port: None,
            default_addr_pool: None,
            force_new_cluster: None,
            subnet_size: None,
            spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmJoinRequest {
    /// Listen address used for inter-manager communication if the node gets promoted to manager, as well as determining the networking interface used for the VXLAN Tunnel Endpoint (VTEP).
    #[serde(rename = "ListenAddr", skip_serializing_if = "Option::is_none")]
    pub listen_addr: Option<String>,
    /// Externally reachable address advertised to other nodes. This can either be an address/port combination in the form `192.168.1.1:4567`, or an interface followed by a port number, like `eth0:4567`. If the port number is omitted, the port number from the listen address is used. If `AdvertiseAddr` is not specified, it will be automatically detected when possible.
    #[serde(rename = "AdvertiseAddr", skip_serializing_if = "Option::is_none")]
    pub advertise_addr: Option<String>,
    /// Address or interface to use for data path traffic (format: `<ip|interface>`), for example,  `192.168.1.1`, or an interface, like `eth0`. If `DataPathAddr` is unspecified, the same address as `AdvertiseAddr` is used.  The `DataPathAddr` specifies the address that global scope network drivers will publish towards other nodes in order to reach the containers running on this node. Using this parameter it is possible to separate the container data traffic from the management traffic of the cluster.
    #[serde(rename = "DataPathAddr", skip_serializing_if = "Option::is_none")]
    pub data_path_addr: Option<String>,
    /// Addresses of manager nodes already participating in the swarm.
    #[serde(rename = "RemoteAddrs", skip_serializing_if = "Option::is_none")]
    pub remote_addrs: Option<Vec<String>>,
    /// Secret token for joining this swarm.
    #[serde(rename = "JoinToken", skip_serializing_if = "Option::is_none")]
    pub join_token: Option<String>,
}
impl SwarmJoinRequest {
    pub fn new() -> SwarmJoinRequest {
        SwarmJoinRequest {
            listen_addr: None,
            advertise_addr: None,
            data_path_addr: None,
            remote_addrs: None,
            join_token: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpec : User modifiable swarm configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpec {
    /// Name of the swarm.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(
        rename = "Orchestration",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub orchestration: Option<Option<Box<models::SwarmSpecOrchestration>>>,
    #[serde(rename = "Raft", skip_serializing_if = "Option::is_none")]
    pub raft: Option<Box<models::SwarmSpecRaft>>,
    #[serde(
        rename = "Dispatcher",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dispatcher: Option<Option<Box<models::SwarmSpecDispatcher>>>,
    #[serde(
        rename = "CAConfig",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ca_config: Option<Option<Box<models::SwarmSpecCaConfig>>>,
    #[serde(rename = "EncryptionConfig", skip_serializing_if = "Option::is_none")]
    pub encryption_config: Option<Box<models::SwarmSpecEncryptionConfig>>,
    #[serde(rename = "TaskDefaults", skip_serializing_if = "Option::is_none")]
    pub task_defaults: Option<Box<models::SwarmSpecTaskDefaults>>,
}
impl SwarmSpec {
    /// User modifiable swarm configuration.
    pub fn new() -> SwarmSpec {
        SwarmSpec {
            name: None,
            labels: None,
            orchestration: None,
            raft: None,
            dispatcher: None,
            ca_config: None,
            encryption_config: None,
            task_defaults: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpecCaConfig : CA configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecCaConfig {
    /// The duration node certificates are issued for.
    #[serde(rename = "NodeCertExpiry", skip_serializing_if = "Option::is_none")]
    pub node_cert_expiry: Option<i64>,
    /// Configuration for forwarding signing requests to an external certificate authority.
    #[serde(rename = "ExternalCAs", skip_serializing_if = "Option::is_none")]
    pub external_cas: Option<Vec<models::SwarmSpecCaConfigExternalCasInner>>,
    /// The desired signing CA certificate for all swarm node TLS leaf certificates, in PEM format.
    #[serde(rename = "SigningCACert", skip_serializing_if = "Option::is_none")]
    pub signing_ca_cert: Option<String>,
    /// The desired signing CA key for all swarm node TLS leaf certificates, in PEM format.
    #[serde(rename = "SigningCAKey", skip_serializing_if = "Option::is_none")]
    pub signing_ca_key: Option<String>,
    /// An integer whose purpose is to force swarm to generate a new signing CA certificate and key, if none have been specified in `SigningCACert` and `SigningCAKey`
    #[serde(rename = "ForceRotate", skip_serializing_if = "Option::is_none")]
    pub force_rotate: Option<i32>,
}
impl SwarmSpecCaConfig {
    /// CA configuration.
    pub fn new() -> SwarmSpecCaConfig {
        SwarmSpecCaConfig {
            node_cert_expiry: None,
            external_cas: None,
            signing_ca_cert: None,
            signing_ca_key: None,
            force_rotate: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecCaConfigExternalCasInner {
    /// Protocol for communication with the external CA (currently only `cfssl` is supported).
    #[serde(rename = "Protocol", skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
    /// URL where certificate signing requests should be sent.
    #[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// An object with key/value pairs that are interpreted as protocol-specific options for the external CA driver.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    /// The root CA certificate (in PEM format) this external CA uses to issue TLS certificates (assumed to be to the current swarm root CA certificate if not provided).
    #[serde(rename = "CACert", skip_serializing_if = "Option::is_none")]
    pub ca_cert: Option<String>,
}
impl SwarmSpecCaConfigExternalCasInner {
    pub fn new() -> SwarmSpecCaConfigExternalCasInner {
        SwarmSpecCaConfigExternalCasInner {
            protocol: None,
            url: None,
            options: None,
            ca_cert: None,
        }
    }
}
/// Protocol for communication with the external CA (currently only `cfssl` is supported).
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Protocol {
    #[serde(rename = "cfssl")]
    Cfssl,
}
impl Default for Protocol {
    fn default() -> Protocol {
        Self::Cfssl
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpecDispatcher : Dispatcher configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecDispatcher {
    /// The delay for an agent to send a heartbeat to the dispatcher.
    #[serde(rename = "HeartbeatPeriod", skip_serializing_if = "Option::is_none")]
    pub heartbeat_period: Option<i64>,
}
impl SwarmSpecDispatcher {
    /// Dispatcher configuration.
    pub fn new() -> SwarmSpecDispatcher {
        SwarmSpecDispatcher {
            heartbeat_period: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpecEncryptionConfig : Parameters related to encryption-at-rest.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecEncryptionConfig {
    /// If set, generate a key and use it to lock data stored on the managers.
    #[serde(rename = "AutoLockManagers", skip_serializing_if = "Option::is_none")]
    pub auto_lock_managers: Option<bool>,
}
impl SwarmSpecEncryptionConfig {
    /// Parameters related to encryption-at-rest.
    pub fn new() -> SwarmSpecEncryptionConfig {
        SwarmSpecEncryptionConfig {
            auto_lock_managers: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpecOrchestration : Orchestration configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecOrchestration {
    /// The number of historic tasks to keep per instance or node. If negative, never remove completed or failed tasks.
    #[serde(
        rename = "TaskHistoryRetentionLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub task_history_retention_limit: Option<i64>,
}
impl SwarmSpecOrchestration {
    /// Orchestration configuration.
    pub fn new() -> SwarmSpecOrchestration {
        SwarmSpecOrchestration {
            task_history_retention_limit: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpecRaft : Raft configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecRaft {
    /// The number of log entries between snapshots.
    #[serde(rename = "SnapshotInterval", skip_serializing_if = "Option::is_none")]
    pub snapshot_interval: Option<i32>,
    /// The number of snapshots to keep beyond the current snapshot.
    #[serde(rename = "KeepOldSnapshots", skip_serializing_if = "Option::is_none")]
    pub keep_old_snapshots: Option<i32>,
    /// The number of log entries to keep around to sync up slow followers after a snapshot is created.
    #[serde(
        rename = "LogEntriesForSlowFollowers",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_entries_for_slow_followers: Option<i32>,
    /// The number of ticks that a follower will wait for a message from the leader before becoming a candidate and starting an election. `ElectionTick` must be greater than `HeartbeatTick`.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[serde(rename = "ElectionTick", skip_serializing_if = "Option::is_none")]
    pub election_tick: Option<i32>,
    /// The number of ticks between heartbeats. Every HeartbeatTick ticks, the leader will send a heartbeat to the followers.  A tick currently defaults to one second, so these translate directly to seconds currently, but this is NOT guaranteed.
    #[serde(rename = "HeartbeatTick", skip_serializing_if = "Option::is_none")]
    pub heartbeat_tick: Option<i32>,
}
impl SwarmSpecRaft {
    /// Raft configuration.
    pub fn new() -> SwarmSpecRaft {
        SwarmSpecRaft {
            snapshot_interval: None,
            keep_old_snapshots: None,
            log_entries_for_slow_followers: None,
            election_tick: None,
            heartbeat_tick: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpecTaskDefaults : Defaults for creating tasks in this cluster.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecTaskDefaults {
    #[serde(rename = "LogDriver", skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<Box<models::SwarmSpecTaskDefaultsLogDriver>>,
}
impl SwarmSpecTaskDefaults {
    /// Defaults for creating tasks in this cluster.
    pub fn new() -> SwarmSpecTaskDefaults {
        SwarmSpecTaskDefaults {
            log_driver: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SwarmSpecTaskDefaultsLogDriver : The log driver to use for tasks created in the orchestrator if unspecified by a service.  Updating this value only affects new tasks. Existing tasks continue to use their previously configured log driver until recreated.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmSpecTaskDefaultsLogDriver {
    /// The log driver to use as a default for new tasks.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Driver-specific options for the selected log driver, specified as key/value pairs.
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
}
impl SwarmSpecTaskDefaultsLogDriver {
    /// The log driver to use for tasks created in the orchestrator if unspecified by a service.  Updating this value only affects new tasks. Existing tasks continue to use their previously configured log driver until recreated.
    pub fn new() -> SwarmSpecTaskDefaultsLogDriver {
        SwarmSpecTaskDefaultsLogDriver {
            name: None,
            options: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SwarmUnlockRequest {
    /// The swarm's unlock key.
    #[serde(rename = "UnlockKey", skip_serializing_if = "Option::is_none")]
    pub unlock_key: Option<String>,
}
impl SwarmUnlockRequest {
    pub fn new() -> SwarmUnlockRequest {
        SwarmUnlockRequest {
            unlock_key: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemAuthResponse {
    /// The status of the authentication
    #[serde(rename = "Status")]
    pub status: String,
    /// An opaque token used to authenticate a user after a successful login
    #[serde(rename = "IdentityToken", skip_serializing_if = "Option::is_none")]
    pub identity_token: Option<String>,
}
impl SystemAuthResponse {
    pub fn new(status: String) -> SystemAuthResponse {
        SystemAuthResponse {
            status,
            identity_token: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemDataUsageResponse {
    #[serde(rename = "LayersSize", skip_serializing_if = "Option::is_none")]
    pub layers_size: Option<i64>,
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<models::ImageSummary>>,
    #[serde(rename = "Containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<models::ContainerSummary>>,
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<models::Volume>>,
    #[serde(rename = "BuildCache", skip_serializing_if = "Option::is_none")]
    pub build_cache: Option<Vec<models::BuildCache>>,
}
impl SystemDataUsageResponse {
    pub fn new() -> SystemDataUsageResponse {
        SystemDataUsageResponse {
            layers_size: None,
            images: None,
            containers: None,
            volumes: None,
            build_cache: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfo {
    /// Unique identifier of the daemon.  <p><br /></p>  > **Note**: The format of the ID itself is not part of the API, and > should not be considered stable.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Total number of containers on the host.
    #[serde(rename = "Containers", skip_serializing_if = "Option::is_none")]
    pub containers: Option<i32>,
    /// Number of containers with status `\"running\"`.
    #[serde(rename = "ContainersRunning", skip_serializing_if = "Option::is_none")]
    pub containers_running: Option<i32>,
    /// Number of containers with status `\"paused\"`.
    #[serde(rename = "ContainersPaused", skip_serializing_if = "Option::is_none")]
    pub containers_paused: Option<i32>,
    /// Number of containers with status `\"stopped\"`.
    #[serde(rename = "ContainersStopped", skip_serializing_if = "Option::is_none")]
    pub containers_stopped: Option<i32>,
    /// Total number of images on the host.  Both _tagged_ and _untagged_ (dangling) images are counted.
    #[serde(rename = "Images", skip_serializing_if = "Option::is_none")]
    pub images: Option<i32>,
    /// Name of the storage driver in use.
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// Information specific to the storage driver, provided as \"label\" / \"value\" pairs.  This information is provided by the storage driver, and formatted in a way consistent with the output of `docker info` on the command line.  <p><br /></p>  > **Note**: The information returned in this field, including the > formatting of values and labels, should not be considered stable, > and may change without notice.
    #[serde(rename = "DriverStatus", skip_serializing_if = "Option::is_none")]
    pub driver_status: Option<Vec<Vec<String>>>,
    /// Root directory of persistent Docker state.  Defaults to `/var/lib/docker` on Linux, and `C:\\ProgramData\\docker` on Windows.
    #[serde(rename = "DockerRootDir", skip_serializing_if = "Option::is_none")]
    pub docker_root_dir: Option<String>,
    #[serde(rename = "Plugins", skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Box<models::PluginsInfo>>,
    /// Indicates if the host has memory limit support enabled.
    #[serde(rename = "MemoryLimit", skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<bool>,
    /// Indicates if the host has memory swap limit support enabled.
    #[serde(rename = "SwapLimit", skip_serializing_if = "Option::is_none")]
    pub swap_limit: Option<bool>,
    /// Indicates if the host has kernel memory TCP limit support enabled. This field is omitted if not supported.  Kernel memory TCP limits are not supported when using cgroups v2, which does not support the corresponding `memory.kmem.tcp.limit_in_bytes` cgroup.
    #[serde(rename = "KernelMemoryTCP", skip_serializing_if = "Option::is_none")]
    pub kernel_memory_tcp: Option<bool>,
    /// Indicates if CPU CFS(Completely Fair Scheduler) period is supported by the host.
    #[serde(rename = "CpuCfsPeriod", skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_period: Option<bool>,
    /// Indicates if CPU CFS(Completely Fair Scheduler) quota is supported by the host.
    #[serde(rename = "CpuCfsQuota", skip_serializing_if = "Option::is_none")]
    pub cpu_cfs_quota: Option<bool>,
    /// Indicates if CPU Shares limiting is supported by the host.
    #[serde(rename = "CPUShares", skip_serializing_if = "Option::is_none")]
    pub cpu_shares: Option<bool>,
    /// Indicates if CPUsets (cpuset.cpus, cpuset.mems) are supported by the host.  See [cpuset(7)](https://www.kernel.org/doc/Documentation/cgroup-v1/cpusets.txt)
    #[serde(rename = "CPUSet", skip_serializing_if = "Option::is_none")]
    pub cpu_set: Option<bool>,
    /// Indicates if the host kernel has PID limit support enabled.
    #[serde(rename = "PidsLimit", skip_serializing_if = "Option::is_none")]
    pub pids_limit: Option<bool>,
    /// Indicates if OOM killer disable is supported on the host.
    #[serde(rename = "OomKillDisable", skip_serializing_if = "Option::is_none")]
    pub oom_kill_disable: Option<bool>,
    /// Indicates IPv4 forwarding is enabled.
    #[serde(rename = "IPv4Forwarding", skip_serializing_if = "Option::is_none")]
    pub ipv4_forwarding: Option<bool>,
    /// Indicates if `bridge-nf-call-iptables` is available on the host.
    #[serde(rename = "BridgeNfIptables", skip_serializing_if = "Option::is_none")]
    pub bridge_nf_iptables: Option<bool>,
    /// Indicates if `bridge-nf-call-ip6tables` is available on the host.
    #[serde(rename = "BridgeNfIp6tables", skip_serializing_if = "Option::is_none")]
    pub bridge_nf_ip6tables: Option<bool>,
    /// Indicates if the daemon is running in debug-mode / with debug-level logging enabled.
    #[serde(rename = "Debug", skip_serializing_if = "Option::is_none")]
    pub debug: Option<bool>,
    /// The total number of file Descriptors in use by the daemon process.  This information is only returned if debug-mode is enabled.
    #[serde(rename = "NFd", skip_serializing_if = "Option::is_none")]
    pub nfd: Option<i32>,
    /// The  number of goroutines that currently exist.  This information is only returned if debug-mode is enabled.
    #[serde(rename = "NGoroutines", skip_serializing_if = "Option::is_none")]
    pub n_goroutines: Option<i32>,
    /// Current system-time in [RFC 3339](https://www.ietf.org/rfc/rfc3339.txt) format with nano-seconds.
    #[serde(rename = "SystemTime", skip_serializing_if = "Option::is_none")]
    pub system_time: Option<String>,
    /// The logging driver to use as a default for new containers.
    #[serde(rename = "LoggingDriver", skip_serializing_if = "Option::is_none")]
    pub logging_driver: Option<String>,
    /// The driver to use for managing cgroups.
    #[serde(rename = "CgroupDriver", skip_serializing_if = "Option::is_none")]
    pub cgroup_driver: Option<CgroupDriver>,
    /// The version of the cgroup.
    #[serde(rename = "CgroupVersion", skip_serializing_if = "Option::is_none")]
    pub cgroup_version: Option<CgroupVersion>,
    /// Number of event listeners subscribed.
    #[serde(rename = "NEventsListener", skip_serializing_if = "Option::is_none")]
    pub n_events_listener: Option<i32>,
    /// Kernel version of the host.  On Linux, this information obtained from `uname`. On Windows this information is queried from the <kbd>HKEY_LOCAL_MACHINE\\\\SOFTWARE\\\\Microsoft\\\\Windows NT\\\\CurrentVersion\\\\</kbd> registry value, for example _\"10.0 14393 (14393.1198.amd64fre.rs1_release_sec.170427-1353)\"_.
    #[serde(rename = "KernelVersion", skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// Name of the host's operating system, for example: \"Ubuntu 24.04 LTS\" or \"Windows Server 2016 Datacenter\"
    #[serde(rename = "OperatingSystem", skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
    /// Version of the host's operating system  <p><br /></p>  > **Note**: The information returned in this field, including its > very existence, and the formatting of values, should not be considered > stable, and may change without notice.
    #[serde(rename = "OSVersion", skip_serializing_if = "Option::is_none")]
    pub os_version: Option<String>,
    /// Generic type of the operating system of the host, as returned by the Go runtime (`GOOS`).  Currently returned values are \"linux\" and \"windows\". A full list of possible values can be found in the [Go documentation](https://go.dev/doc/install/source#environment).
    #[serde(rename = "OSType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<String>,
    /// Hardware architecture of the host, as returned by the Go runtime (`GOARCH`).  A full list of possible values can be found in the [Go documentation](https://go.dev/doc/install/source#environment).
    #[serde(rename = "Architecture", skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    /// The number of logical CPUs usable by the daemon.  The number of available CPUs is checked by querying the operating system when the daemon starts. Changes to operating system CPU allocation after the daemon is started are not reflected.
    #[serde(rename = "NCPU", skip_serializing_if = "Option::is_none")]
    pub ncpu: Option<i32>,
    /// Total amount of physical memory available on the host, in bytes.
    #[serde(rename = "MemTotal", skip_serializing_if = "Option::is_none")]
    pub mem_total: Option<i64>,
    /// Address / URL of the index server that is used for image search, and as a default for user authentication for Docker Hub and Docker Cloud.
    #[serde(rename = "IndexServerAddress", skip_serializing_if = "Option::is_none")]
    pub index_server_address: Option<String>,
    #[serde(
        rename = "RegistryConfig",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub registry_config: Option<Option<Box<models::RegistryServiceConfig>>>,
    /// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, `GPU=UUID1`).
    #[serde(rename = "GenericResources", skip_serializing_if = "Option::is_none")]
    pub generic_resources: Option<Vec<models::GenericResourcesInner>>,
    /// HTTP-proxy configured for the daemon. This value is obtained from the [`HTTP_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration.
    #[serde(rename = "HttpProxy", skip_serializing_if = "Option::is_none")]
    pub http_proxy: Option<String>,
    /// HTTPS-proxy configured for the daemon. This value is obtained from the [`HTTPS_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable. Credentials ([user info component](https://tools.ietf.org/html/rfc3986#section-3.2.1)) in the proxy URL are masked in the API response.  Containers do not automatically inherit this configuration.
    #[serde(rename = "HttpsProxy", skip_serializing_if = "Option::is_none")]
    pub https_proxy: Option<String>,
    /// Comma-separated list of domain extensions for which no proxy should be used. This value is obtained from the [`NO_PROXY`](https://www.gnu.org/software/wget/manual/html_node/Proxies.html) environment variable.  Containers do not automatically inherit this configuration.
    #[serde(rename = "NoProxy", skip_serializing_if = "Option::is_none")]
    pub no_proxy: Option<String>,
    /// Hostname of the host.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined labels (key/value metadata) as set on the daemon.  <p><br /></p>  > **Note**: When part of a Swarm, nodes can both have _daemon_ labels, > set through the daemon configuration, and _node_ labels, set from a > manager node in the Swarm. Node labels are not included in this > field. Node labels can be retrieved using the `/nodes/(id)` endpoint > on a manager node in the Swarm.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
    /// Indicates if experimental features are enabled on the daemon.
    #[serde(rename = "ExperimentalBuild", skip_serializing_if = "Option::is_none")]
    pub experimental_build: Option<bool>,
    /// Version string of the daemon.
    #[serde(rename = "ServerVersion", skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    /// List of [OCI compliant](https://github.com/opencontainers/runtime-spec) runtimes configured on the daemon. Keys hold the \"name\" used to reference the runtime.  The Docker daemon relies on an OCI compliant runtime (invoked via the `containerd` daemon) as its interface to the Linux kernel namespaces, cgroups, and SELinux.  The default runtime is `runc`, and automatically configured. Additional runtimes can be configured by the user and will be listed here.
    #[serde(rename = "Runtimes", skip_serializing_if = "Option::is_none")]
    pub runtimes: Option<std::collections::HashMap<String, models::Runtime>>,
    /// Name of the default OCI runtime that is used when starting containers.  The default can be overridden per-container at create time.
    #[serde(rename = "DefaultRuntime", skip_serializing_if = "Option::is_none")]
    pub default_runtime: Option<String>,
    #[serde(rename = "Swarm", skip_serializing_if = "Option::is_none")]
    pub swarm: Option<Box<models::SwarmInfo>>,
    /// Indicates if live restore is enabled.  If enabled, containers are kept running when the daemon is shutdown or upon daemon start if running containers are detected.
    #[serde(rename = "LiveRestoreEnabled", skip_serializing_if = "Option::is_none")]
    pub live_restore_enabled: Option<bool>,
    /// Represents the isolation technology to use as a default for containers. The supported values are platform-specific.  If no isolation value is specified on daemon start, on Windows client, the default is `hyperv`, and on Windows server, the default is `process`.  This option is currently not used on other platforms.
    #[serde(rename = "Isolation", skip_serializing_if = "Option::is_none")]
    pub isolation: Option<Isolation>,
    /// Name and, optional, path of the `docker-init` binary.  If the path is omitted, the daemon searches the host's `$PATH` for the binary and uses the first result.
    #[serde(rename = "InitBinary", skip_serializing_if = "Option::is_none")]
    pub init_binary: Option<String>,
    #[serde(rename = "ContainerdCommit", skip_serializing_if = "Option::is_none")]
    pub containerd_commit: Option<Box<models::Commit>>,
    #[serde(rename = "RuncCommit", skip_serializing_if = "Option::is_none")]
    pub runc_commit: Option<Box<models::Commit>>,
    #[serde(rename = "InitCommit", skip_serializing_if = "Option::is_none")]
    pub init_commit: Option<Box<models::Commit>>,
    /// List of security features that are enabled on the daemon, such as apparmor, seccomp, SELinux, user-namespaces (userns), rootless and no-new-privileges.  Additional configuration options for each security feature may be present, and are included as a comma-separated list of key/value pairs.
    #[serde(rename = "SecurityOptions", skip_serializing_if = "Option::is_none")]
    pub security_options: Option<Vec<String>>,
    /// Reports a summary of the product license on the daemon.  If a commercial license has been applied to the daemon, information such as number of nodes, and expiration are included.
    #[serde(rename = "ProductLicense", skip_serializing_if = "Option::is_none")]
    pub product_license: Option<String>,
    /// List of custom default address pools for local networks, which can be specified in the daemon.json file or dockerd option.  Example: a Base \"10.10.0.0/16\" with Size 24 will define the set of 256 10.10.[0-255].0/24 address pools.
    #[serde(rename = "DefaultAddressPools", skip_serializing_if = "Option::is_none")]
    pub default_address_pools: Option<Vec<models::SystemInfoDefaultAddressPoolsInner>>,
    /// List of warnings / informational messages about missing features, or issues related to the daemon configuration.  These messages can be printed by the client as information to the user.
    #[serde(rename = "Warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
    /// List of directories where (Container Device Interface) CDI specifications are located.  These specifications define vendor-specific modifications to an OCI runtime specification for a container being created.  An empty list indicates that CDI device injection is disabled.  Note that since using CDI device injection requires the daemon to have experimental enabled. For non-experimental daemons an empty list will always be returned.
    #[serde(rename = "CDISpecDirs", skip_serializing_if = "Option::is_none")]
    pub cdi_spec_dirs: Option<Vec<String>>,
    #[serde(
        rename = "Containerd",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub containerd: Option<Option<Box<models::ContainerdInfo>>>,
}
impl SystemInfo {
    pub fn new() -> SystemInfo {
        SystemInfo {
            id: None,
            containers: None,
            containers_running: None,
            containers_paused: None,
            containers_stopped: None,
            images: None,
            driver: None,
            driver_status: None,
            docker_root_dir: None,
            plugins: None,
            memory_limit: None,
            swap_limit: None,
            kernel_memory_tcp: None,
            cpu_cfs_period: None,
            cpu_cfs_quota: None,
            cpu_shares: None,
            cpu_set: None,
            pids_limit: None,
            oom_kill_disable: None,
            ipv4_forwarding: None,
            bridge_nf_iptables: None,
            bridge_nf_ip6tables: None,
            debug: None,
            nfd: None,
            n_goroutines: None,
            system_time: None,
            logging_driver: None,
            cgroup_driver: None,
            cgroup_version: None,
            n_events_listener: None,
            kernel_version: None,
            operating_system: None,
            os_version: None,
            os_type: None,
            architecture: None,
            ncpu: None,
            mem_total: None,
            index_server_address: None,
            registry_config: None,
            generic_resources: None,
            http_proxy: None,
            https_proxy: None,
            no_proxy: None,
            name: None,
            labels: None,
            experimental_build: None,
            server_version: None,
            runtimes: None,
            default_runtime: None,
            swarm: None,
            live_restore_enabled: None,
            isolation: None,
            init_binary: None,
            containerd_commit: None,
            runc_commit: None,
            init_commit: None,
            security_options: None,
            product_license: None,
            default_address_pools: None,
            warnings: None,
            cdi_spec_dirs: None,
            containerd: None,
        }
    }
}
/// The driver to use for managing cgroups.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum CgroupDriver {
    #[serde(rename = "cgroupfs")]
    Cgroupfs,
    #[serde(rename = "systemd")]
    Systemd,
    #[serde(rename = "none")]
    None,
}
impl Default for CgroupDriver {
    fn default() -> CgroupDriver {
        Self::Cgroupfs
    }
}
/// The version of the cgroup.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum CgroupVersion {
    #[serde(rename = "1")]
    Variant1,
    #[serde(rename = "2")]
    Variant2,
}
impl Default for CgroupVersion {
    fn default() -> CgroupVersion {
        Self::Variant1
    }
}
/// Represents the isolation technology to use as a default for containers. The supported values are platform-specific.  If no isolation value is specified on daemon start, on Windows client, the default is `hyperv`, and on Windows server, the default is `process`.  This option is currently not used on other platforms.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Isolation {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "hyperv")]
    Hyperv,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "")]
    Empty,
}
impl Default for Isolation {
    fn default() -> Isolation {
        Self::Default
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemInfoDefaultAddressPoolsInner {
    /// The network address in CIDR format
    #[serde(rename = "Base", skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// The network pool size
    #[serde(rename = "Size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}
impl SystemInfoDefaultAddressPoolsInner {
    pub fn new() -> SystemInfoDefaultAddressPoolsInner {
        SystemInfoDefaultAddressPoolsInner {
            base: None,
            size: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// SystemVersion : Response of Engine API: GET \"/version\"
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemVersion {
    #[serde(rename = "Platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<Box<models::SystemVersionPlatform>>,
    /// Information about system components
    #[serde(rename = "Components", skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<models::SystemVersionComponentsInner>>,
    /// The version of the daemon
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// The default (and highest) API version that is supported by the daemon
    #[serde(rename = "ApiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The minimum API version that is supported by the daemon
    #[serde(rename = "MinAPIVersion", skip_serializing_if = "Option::is_none")]
    pub min_api_version: Option<String>,
    /// The Git commit of the source code that was used to build the daemon
    #[serde(rename = "GitCommit", skip_serializing_if = "Option::is_none")]
    pub git_commit: Option<String>,
    /// The version Go used to compile the daemon, and the version of the Go runtime in use.
    #[serde(rename = "GoVersion", skip_serializing_if = "Option::is_none")]
    pub go_version: Option<String>,
    /// The operating system that the daemon is running on (\"linux\" or \"windows\")
    #[serde(rename = "Os", skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The architecture that the daemon is running on
    #[serde(rename = "Arch", skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    /// The kernel version (`uname -r`) that the daemon is running on.  This field is omitted when empty.
    #[serde(rename = "KernelVersion", skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// Indicates if the daemon is started with experimental features enabled.  This field is omitted when empty / false.
    #[serde(rename = "Experimental", skip_serializing_if = "Option::is_none")]
    pub experimental: Option<bool>,
    /// The date and time that the daemon was compiled.
    #[serde(rename = "BuildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
}
impl SystemVersion {
    /// Response of Engine API: GET \"/version\"
    pub fn new() -> SystemVersion {
        SystemVersion {
            platform: None,
            components: None,
            version: None,
            api_version: None,
            min_api_version: None,
            git_commit: None,
            go_version: None,
            os: None,
            arch: None,
            kernel_version: None,
            experimental: None,
            build_time: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemVersionComponentsInner {
    /// Name of the component
    #[serde(rename = "Name")]
    pub name: String,
    /// Version of the component
    #[serde(rename = "Version")]
    pub version: String,
    /// Key/value pairs of strings with additional information about the component. These values are intended for informational purposes only, and their content is not defined, and not part of the API specification.  These messages can be printed by the client as information to the user.
    #[serde(
        rename = "Details",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub details: Option<Option<serde_json::Value>>,
}
impl SystemVersionComponentsInner {
    pub fn new(name: String, version: String) -> SystemVersionComponentsInner {
        SystemVersionComponentsInner {
            name,
            version,
            details: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SystemVersionPlatform {
    #[serde(rename = "Name")]
    pub name: String,
}
impl SystemVersionPlatform {
    pub fn new(name: String) -> SystemVersionPlatform {
        SystemVersionPlatform { name }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Task {
    /// The ID of the task.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Version", skip_serializing_if = "Option::is_none")]
    pub version: Option<Box<models::ObjectVersion>>,
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "UpdatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Name of the task.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::TaskSpec>>,
    /// The ID of the service this task is part of.
    #[serde(rename = "ServiceID", skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(rename = "Slot", skip_serializing_if = "Option::is_none")]
    pub slot: Option<i32>,
    /// The ID of the node that this task is on.
    #[serde(rename = "NodeID", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// User-defined resources can be either Integer resources (e.g, `SSD=3`) or String resources (e.g, `GPU=UUID1`).
    #[serde(
        rename = "AssignedGenericResources",
        skip_serializing_if = "Option::is_none"
    )]
    pub assigned_generic_resources: Option<Vec<models::GenericResourcesInner>>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Box<models::TaskStatus>>,
    #[serde(rename = "DesiredState", skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<models::TaskState>,
    #[serde(rename = "JobIteration", skip_serializing_if = "Option::is_none")]
    pub job_iteration: Option<Box<models::ObjectVersion>>,
}
impl Task {
    pub fn new() -> Task {
        Task {
            id: None,
            version: None,
            created_at: None,
            updated_at: None,
            name: None,
            labels: None,
            spec: None,
            service_id: None,
            slot: None,
            node_id: None,
            assigned_generic_resources: None,
            status: None,
            desired_state: None,
            job_iteration: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpec : User modifiable task configuration.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpec {
    #[serde(rename = "PluginSpec", skip_serializing_if = "Option::is_none")]
    pub plugin_spec: Option<Box<models::TaskSpecPluginSpec>>,
    #[serde(rename = "ContainerSpec", skip_serializing_if = "Option::is_none")]
    pub container_spec: Option<Box<models::TaskSpecContainerSpec>>,
    #[serde(rename = "NetworkAttachmentSpec", skip_serializing_if = "Option::is_none")]
    pub network_attachment_spec: Option<Box<models::TaskSpecNetworkAttachmentSpec>>,
    #[serde(rename = "Resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<models::TaskSpecResources>>,
    #[serde(rename = "RestartPolicy", skip_serializing_if = "Option::is_none")]
    pub restart_policy: Option<Box<models::TaskSpecRestartPolicy>>,
    #[serde(rename = "Placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<Box<models::TaskSpecPlacement>>,
    /// A counter that triggers an update even if no relevant parameters have been changed.
    #[serde(rename = "ForceUpdate", skip_serializing_if = "Option::is_none")]
    pub force_update: Option<i32>,
    /// Runtime is the type of runtime specified for the task executor.
    #[serde(rename = "Runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// Specifies which networks the service should attach to.
    #[serde(rename = "Networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<models::NetworkAttachmentConfig>>,
    #[serde(rename = "LogDriver", skip_serializing_if = "Option::is_none")]
    pub log_driver: Option<Box<models::TaskSpecLogDriver>>,
}
impl TaskSpec {
    /// User modifiable task configuration.
    pub fn new() -> TaskSpec {
        TaskSpec {
            plugin_spec: None,
            container_spec: None,
            network_attachment_spec: None,
            resources: None,
            restart_policy: None,
            placement: None,
            force_update: None,
            runtime: None,
            networks: None,
            log_driver: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpec : Container spec for the service.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpec {
    /// The image name to use for the container
    #[serde(rename = "Image", skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// User-defined key/value data.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    /// The command to be run in the image.
    #[serde(rename = "Command", skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// Arguments to the command.
    #[serde(rename = "Args", skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// The hostname to use for the container, as a valid [RFC 1123](https://tools.ietf.org/html/rfc1123) hostname.
    #[serde(rename = "Hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// A list of environment variables in the form `VAR=value`.
    #[serde(rename = "Env", skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    /// The working directory for commands to run in.
    #[serde(rename = "Dir", skip_serializing_if = "Option::is_none")]
    pub dir: Option<String>,
    /// The user inside the container.
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// A list of additional groups that the container process will run as.
    #[serde(rename = "Groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "Privileges", skip_serializing_if = "Option::is_none")]
    pub privileges: Option<Box<models::TaskSpecContainerSpecPrivileges>>,
    /// Whether a pseudo-TTY should be allocated.
    #[serde(rename = "TTY", skip_serializing_if = "Option::is_none")]
    pub tty: Option<bool>,
    /// Open `stdin`
    #[serde(rename = "OpenStdin", skip_serializing_if = "Option::is_none")]
    pub open_stdin: Option<bool>,
    /// Mount the container's root filesystem as read only.
    #[serde(rename = "ReadOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// Specification for mounts to be added to containers created as part of the service.
    #[serde(rename = "Mounts", skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<models::Mount>>,
    /// Signal to stop the container.
    #[serde(rename = "StopSignal", skip_serializing_if = "Option::is_none")]
    pub stop_signal: Option<String>,
    /// Amount of time to wait for the container to terminate before forcefully killing it.
    #[serde(rename = "StopGracePeriod", skip_serializing_if = "Option::is_none")]
    pub stop_grace_period: Option<i64>,
    #[serde(rename = "HealthCheck", skip_serializing_if = "Option::is_none")]
    pub health_check: Option<Box<models::HealthConfig>>,
    /// A list of hostname/IP mappings to add to the container's `hosts` file. The format of extra hosts is specified in the [hosts(5)](http://man7.org/linux/man-pages/man5/hosts.5.html) man page:      IP_address canonical_hostname [aliases...]
    #[serde(rename = "Hosts", skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "DNSConfig", skip_serializing_if = "Option::is_none")]
    pub dns_config: Option<Box<models::TaskSpecContainerSpecDnsConfig>>,
    /// Secrets contains references to zero or more secrets that will be exposed to the service.
    #[serde(rename = "Secrets", skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<models::TaskSpecContainerSpecSecretsInner>>,
    /// An integer value containing the score given to the container in order to tune OOM killer preferences.
    #[serde(rename = "OomScoreAdj", skip_serializing_if = "Option::is_none")]
    pub oom_score_adj: Option<i64>,
    /// Configs contains references to zero or more configs that will be exposed to the service.
    #[serde(rename = "Configs", skip_serializing_if = "Option::is_none")]
    pub configs: Option<Vec<models::TaskSpecContainerSpecConfigsInner>>,
    /// Isolation technology of the containers running the service. (Windows only)
    #[serde(rename = "Isolation", skip_serializing_if = "Option::is_none")]
    pub isolation: Option<Isolation>,
    /// Run an init inside the container that forwards signals and reaps processes. This field is omitted if empty, and the default (as configured on the daemon) is used.
    #[serde(
        rename = "Init",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub init: Option<Option<bool>>,
    /// Set kernel namedspaced parameters (sysctls) in the container. The Sysctls option on services accepts the same sysctls as the are supported on containers. Note that while the same sysctls are supported, no guarantees or checks are made about their suitability for a clustered environment, and it's up to the user to determine whether a given sysctl will work properly in a Service.
    #[serde(rename = "Sysctls", skip_serializing_if = "Option::is_none")]
    pub sysctls: Option<std::collections::HashMap<String, String>>,
    /// A list of kernel capabilities to add to the default set for the container.
    #[serde(rename = "CapabilityAdd", skip_serializing_if = "Option::is_none")]
    pub capability_add: Option<Vec<String>>,
    /// A list of kernel capabilities to drop from the default set for the container.
    #[serde(rename = "CapabilityDrop", skip_serializing_if = "Option::is_none")]
    pub capability_drop: Option<Vec<String>>,
    /// A list of resource limits to set in the container. For example: `{\"Name\": \"nofile\", \"Soft\": 1024, \"Hard\": 2048}`\"
    #[serde(rename = "Ulimits", skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<models::ResourcesUlimitsInner>>,
}
impl TaskSpecContainerSpec {
    /// Container spec for the service.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
    pub fn new() -> TaskSpecContainerSpec {
        TaskSpecContainerSpec {
            image: None,
            labels: None,
            command: None,
            args: None,
            hostname: None,
            env: None,
            dir: None,
            user: None,
            groups: None,
            privileges: None,
            tty: None,
            open_stdin: None,
            read_only: None,
            mounts: None,
            stop_signal: None,
            stop_grace_period: None,
            health_check: None,
            hosts: None,
            dns_config: None,
            secrets: None,
            oom_score_adj: None,
            configs: None,
            isolation: None,
            init: None,
            sysctls: None,
            capability_add: None,
            capability_drop: None,
            ulimits: None,
        }
    }
}
/// Isolation technology of the containers running the service. (Windows only)
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Isolation {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "process")]
    Process,
    #[serde(rename = "hyperv")]
    Hyperv,
    #[serde(rename = "")]
    Empty,
}
impl Default for Isolation {
    fn default() -> Isolation {
        Self::Default
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecConfigsInner {
    #[serde(rename = "File", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<models::TaskSpecContainerSpecConfigsInnerFile>>,
    /// Runtime represents a target that is not mounted into the container but is used by the task  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually > exclusive
    #[serde(rename = "Runtime", skip_serializing_if = "Option::is_none")]
    pub runtime: Option<serde_json::Value>,
    /// ConfigID represents the ID of the specific config that we're referencing.
    #[serde(rename = "ConfigID", skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    /// ConfigName is the name of the config that this references, but this is just provided for lookup/display purposes. The config in the reference will be identified by its ID.
    #[serde(rename = "ConfigName", skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
}
impl TaskSpecContainerSpecConfigsInner {
    pub fn new() -> TaskSpecContainerSpecConfigsInner {
        TaskSpecContainerSpecConfigsInner {
            file: None,
            runtime: None,
            config_id: None,
            config_name: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecConfigsInnerFile : File represents a specific target that is backed by a file.  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually exclusive
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecConfigsInnerFile {
    /// Name represents the final filename in the filesystem.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// UID represents the file UID.
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// GID represents the file GID.
    #[serde(rename = "GID", skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    /// Mode represents the FileMode of the file.
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}
impl TaskSpecContainerSpecConfigsInnerFile {
    /// File represents a specific target that is backed by a file.  <p><br /><p>  > **Note**: `Configs.File` and `Configs.Runtime` are mutually exclusive
    pub fn new() -> TaskSpecContainerSpecConfigsInnerFile {
        TaskSpecContainerSpecConfigsInnerFile {
            name: None,
            uid: None,
            gid: None,
            mode: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecDnsConfig : Specification for DNS related configurations in resolver configuration file (`resolv.conf`).
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecDnsConfig {
    /// The IP addresses of the name servers.
    #[serde(rename = "Nameservers", skip_serializing_if = "Option::is_none")]
    pub nameservers: Option<Vec<String>>,
    /// A search list for host-name lookup.
    #[serde(rename = "Search", skip_serializing_if = "Option::is_none")]
    pub search: Option<Vec<String>>,
    /// A list of internal resolver variables to be modified (e.g., `debug`, `ndots:3`, etc.).
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}
impl TaskSpecContainerSpecDnsConfig {
    /// Specification for DNS related configurations in resolver configuration file (`resolv.conf`).
    pub fn new() -> TaskSpecContainerSpecDnsConfig {
        TaskSpecContainerSpecDnsConfig {
            nameservers: None,
            search: None,
            options: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecPrivileges : Security options for the container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecPrivileges {
    #[serde(rename = "CredentialSpec", skip_serializing_if = "Option::is_none")]
    pub credential_spec: Option<
        Box<models::TaskSpecContainerSpecPrivilegesCredentialSpec>,
    >,
    #[serde(rename = "SELinuxContext", skip_serializing_if = "Option::is_none")]
    pub se_linux_context: Option<
        Box<models::TaskSpecContainerSpecPrivilegesSeLinuxContext>,
    >,
    #[serde(rename = "Seccomp", skip_serializing_if = "Option::is_none")]
    pub seccomp: Option<Box<models::TaskSpecContainerSpecPrivilegesSeccomp>>,
    #[serde(rename = "AppArmor", skip_serializing_if = "Option::is_none")]
    pub app_armor: Option<Box<models::TaskSpecContainerSpecPrivilegesAppArmor>>,
    /// Configuration of the no_new_privs bit in the container
    #[serde(rename = "NoNewPrivileges", skip_serializing_if = "Option::is_none")]
    pub no_new_privileges: Option<bool>,
}
impl TaskSpecContainerSpecPrivileges {
    /// Security options for the container
    pub fn new() -> TaskSpecContainerSpecPrivileges {
        TaskSpecContainerSpecPrivileges {
            credential_spec: None,
            se_linux_context: None,
            seccomp: None,
            app_armor: None,
            no_new_privileges: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecPrivilegesAppArmor : Options for configuring AppArmor on the container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecPrivilegesAppArmor {
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
}
impl TaskSpecContainerSpecPrivilegesAppArmor {
    /// Options for configuring AppArmor on the container
    pub fn new() -> TaskSpecContainerSpecPrivilegesAppArmor {
        TaskSpecContainerSpecPrivilegesAppArmor {
            mode: None,
        }
    }
}
///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Mode {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "disabled")]
    Disabled,
}
impl Default for Mode {
    fn default() -> Mode {
        Self::Default
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecPrivilegesCredentialSpec : CredentialSpec for managed service account (Windows only)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecPrivilegesCredentialSpec {
    /// Load credential spec from a Swarm Config with the given ID. The specified config must also be present in the Configs field with the Runtime property set.  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[serde(rename = "Config", skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    /// Load credential spec from this file. The file is read by the daemon, and must be present in the `CredentialSpecs` subdirectory in the docker data directory, which defaults to `C:\\ProgramData\\Docker\\` on Windows.  For example, specifying `spec.json` loads `C:\\ProgramData\\Docker\\CredentialSpecs\\spec.json`.  <p><br /></p>  > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[serde(rename = "File", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// Load credential spec from this value in the Windows registry. The specified registry value must be located in:  `HKLM\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Virtualization\\Containers\\CredentialSpecs`  <p><br /></p>   > **Note**: `CredentialSpec.File`, `CredentialSpec.Registry`, > and `CredentialSpec.Config` are mutually exclusive.
    #[serde(rename = "Registry", skip_serializing_if = "Option::is_none")]
    pub registry: Option<String>,
}
impl TaskSpecContainerSpecPrivilegesCredentialSpec {
    /// CredentialSpec for managed service account (Windows only)
    pub fn new() -> TaskSpecContainerSpecPrivilegesCredentialSpec {
        TaskSpecContainerSpecPrivilegesCredentialSpec {
            config: None,
            file: None,
            registry: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecPrivilegesSeLinuxContext : SELinux labels of the container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// Disable SELinux
    #[serde(rename = "Disable", skip_serializing_if = "Option::is_none")]
    pub disable: Option<bool>,
    /// SELinux user label
    #[serde(rename = "User", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// SELinux role label
    #[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// SELinux type label
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// SELinux level label
    #[serde(rename = "Level", skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}
impl TaskSpecContainerSpecPrivilegesSeLinuxContext {
    /// SELinux labels of the container
    pub fn new() -> TaskSpecContainerSpecPrivilegesSeLinuxContext {
        TaskSpecContainerSpecPrivilegesSeLinuxContext {
            disable: None,
            user: None,
            role: None,
            r#type: None,
            level: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecPrivilegesSeccomp : Options for configuring seccomp on the container
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecPrivilegesSeccomp {
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// The custom seccomp profile as a json object
    #[serde(rename = "Profile", skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}
impl TaskSpecContainerSpecPrivilegesSeccomp {
    /// Options for configuring seccomp on the container
    pub fn new() -> TaskSpecContainerSpecPrivilegesSeccomp {
        TaskSpecContainerSpecPrivilegesSeccomp {
            mode: None,
            profile: None,
        }
    }
}
///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Mode {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "unconfined")]
    Unconfined,
    #[serde(rename = "custom")]
    Custom,
}
impl Default for Mode {
    fn default() -> Mode {
        Self::Default
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecSecretsInner {
    #[serde(rename = "File", skip_serializing_if = "Option::is_none")]
    pub file: Option<Box<models::TaskSpecContainerSpecSecretsInnerFile>>,
    /// SecretID represents the ID of the specific secret that we're referencing.
    #[serde(rename = "SecretID", skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
    /// SecretName is the name of the secret that this references, but this is just provided for lookup/display purposes. The secret in the reference will be identified by its ID.
    #[serde(rename = "SecretName", skip_serializing_if = "Option::is_none")]
    pub secret_name: Option<String>,
}
impl TaskSpecContainerSpecSecretsInner {
    pub fn new() -> TaskSpecContainerSpecSecretsInner {
        TaskSpecContainerSpecSecretsInner {
            file: None,
            secret_id: None,
            secret_name: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecContainerSpecSecretsInnerFile : File represents a specific target that is backed by a file.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecContainerSpecSecretsInnerFile {
    /// Name represents the final filename in the filesystem.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// UID represents the file UID.
    #[serde(rename = "UID", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    /// GID represents the file GID.
    #[serde(rename = "GID", skip_serializing_if = "Option::is_none")]
    pub gid: Option<String>,
    /// Mode represents the FileMode of the file.
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<i32>,
}
impl TaskSpecContainerSpecSecretsInnerFile {
    /// File represents a specific target that is backed by a file.
    pub fn new() -> TaskSpecContainerSpecSecretsInnerFile {
        TaskSpecContainerSpecSecretsInnerFile {
            name: None,
            uid: None,
            gid: None,
            mode: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecLogDriver : Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecLogDriver {
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
}
impl TaskSpecLogDriver {
    /// Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified.
    pub fn new() -> TaskSpecLogDriver {
        TaskSpecLogDriver {
            name: None,
            options: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecNetworkAttachmentSpec : Read-only spec type for non-swarm containers attached to swarm overlay networks.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecNetworkAttachmentSpec {
    /// ID of the container represented by this task
    #[serde(rename = "ContainerID", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
}
impl TaskSpecNetworkAttachmentSpec {
    /// Read-only spec type for non-swarm containers attached to swarm overlay networks.  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
    pub fn new() -> TaskSpecNetworkAttachmentSpec {
        TaskSpecNetworkAttachmentSpec {
            container_id: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecPlacement {
    /// An array of constraint expressions to limit the set of nodes where a task can be scheduled. Constraint expressions can either use a _match_ (`==`) or _exclude_ (`!=`) rule. Multiple constraints find nodes that satisfy every expression (AND match). Constraints can match node or Docker Engine labels as follows:  node attribute       | matches                        | example ---------------------|--------------------------------|----------------------------------------------- `node.id`            | Node ID                        | `node.id==2ivku8v2gvtg4` `node.hostname`      | Node hostname                  | `node.hostname!=node-2` `node.role`          | Node role (`manager`/`worker`) | `node.role==manager` `node.platform.os`   | Node operating system          | `node.platform.os==windows` `node.platform.arch` | Node architecture              | `node.platform.arch==x86_64` `node.labels`        | User-defined node labels       | `node.labels.security==high` `engine.labels`      | Docker Engine's labels         | `engine.labels.operatingsystem==ubuntu-24.04`  `engine.labels` apply to Docker Engine labels like operating system, drivers, etc. Swarm administrators add `node.labels` for operational purposes by using the [`node update endpoint`](#operation/NodeUpdate).
    #[serde(rename = "Constraints", skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Vec<String>>,
    /// Preferences provide a way to make the scheduler aware of factors such as topology. They are provided in order from highest to lowest precedence.
    #[serde(rename = "Preferences", skip_serializing_if = "Option::is_none")]
    pub preferences: Option<Vec<models::TaskSpecPlacementPreferencesInner>>,
    /// Maximum number of replicas for per node (default value is 0, which is unlimited)
    #[serde(rename = "MaxReplicas", skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<i64>,
    /// Platforms stores all the platforms that the service's image can run on. This field is used in the platform filter for scheduling. If empty, then the platform filter is off, meaning there are no scheduling restrictions.
    #[serde(rename = "Platforms", skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<models::Platform>>,
}
impl TaskSpecPlacement {
    pub fn new() -> TaskSpecPlacement {
        TaskSpecPlacement {
            constraints: None,
            preferences: None,
            max_replicas: None,
            platforms: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecPlacementPreferencesInner {
    #[serde(rename = "Spread", skip_serializing_if = "Option::is_none")]
    pub spread: Option<Box<models::TaskSpecPlacementPreferencesInnerSpread>>,
}
impl TaskSpecPlacementPreferencesInner {
    pub fn new() -> TaskSpecPlacementPreferencesInner {
        TaskSpecPlacementPreferencesInner {
            spread: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecPlacementPreferencesInnerSpread {
    /// label descriptor, such as `engine.labels.az`.
    #[serde(rename = "SpreadDescriptor", skip_serializing_if = "Option::is_none")]
    pub spread_descriptor: Option<String>,
}
impl TaskSpecPlacementPreferencesInnerSpread {
    pub fn new() -> TaskSpecPlacementPreferencesInnerSpread {
        TaskSpecPlacementPreferencesInnerSpread {
            spread_descriptor: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecPluginSpec : Plugin spec for the service.  *(Experimental release only.)*  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecPluginSpec {
    /// The name or 'alias' to use for the plugin.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The plugin image reference to use.
    #[serde(rename = "Remote", skip_serializing_if = "Option::is_none")]
    pub remote: Option<String>,
    /// Disable the plugin once scheduled.
    #[serde(rename = "Disabled", skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "PluginPrivilege", skip_serializing_if = "Option::is_none")]
    pub plugin_privilege: Option<Vec<models::PluginPrivilege>>,
}
impl TaskSpecPluginSpec {
    /// Plugin spec for the service.  *(Experimental release only.)*  <p><br /></p>  > **Note**: ContainerSpec, NetworkAttachmentSpec, and PluginSpec are > mutually exclusive. PluginSpec is only used when the Runtime field > is set to `plugin`. NetworkAttachmentSpec is used when the Runtime > field is set to `attachment`.
    pub fn new() -> TaskSpecPluginSpec {
        TaskSpecPluginSpec {
            name: None,
            remote: None,
            disabled: None,
            plugin_privilege: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecResources : Resource requirements which apply to each individual container created as part of the service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecResources {
    #[serde(rename = "Limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<Box<models::Limit>>,
    #[serde(rename = "Reservations", skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Box<models::ResourceObject>>,
}
impl TaskSpecResources {
    /// Resource requirements which apply to each individual container created as part of the service.
    pub fn new() -> TaskSpecResources {
        TaskSpecResources {
            limits: None,
            reservations: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskSpecRestartPolicy : Specification for the restart policy which applies to containers created as part of this service.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskSpecRestartPolicy {
    /// Condition for restart.
    #[serde(rename = "Condition", skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    /// Delay between restart attempts.
    #[serde(rename = "Delay", skip_serializing_if = "Option::is_none")]
    pub delay: Option<i64>,
    /// Maximum attempts to restart a given container before giving up (default value is 0, which is ignored).
    #[serde(rename = "MaxAttempts", skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    /// Windows is the time window used to evaluate the restart policy (default value is 0, which is unbounded).
    #[serde(rename = "Window", skip_serializing_if = "Option::is_none")]
    pub window: Option<i64>,
}
impl TaskSpecRestartPolicy {
    /// Specification for the restart policy which applies to containers created as part of this service.
    pub fn new() -> TaskSpecRestartPolicy {
        TaskSpecRestartPolicy {
            condition: None,
            delay: None,
            max_attempts: None,
            window: None,
        }
    }
}
/// Condition for restart.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Condition {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "on-failure")]
    OnFailure,
    #[serde(rename = "any")]
    Any,
}
impl Default for Condition {
    fn default() -> Condition {
        Self::None
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
///
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum TaskState {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "allocated")]
    Allocated,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "assigned")]
    Assigned,
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "shutdown")]
    Shutdown,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "remove")]
    Remove,
    #[serde(rename = "orphaned")]
    Orphaned,
}
impl std::fmt::Display for TaskState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::New => write!(f, "new"),
            Self::Allocated => write!(f, "allocated"),
            Self::Pending => write!(f, "pending"),
            Self::Assigned => write!(f, "assigned"),
            Self::Accepted => write!(f, "accepted"),
            Self::Preparing => write!(f, "preparing"),
            Self::Ready => write!(f, "ready"),
            Self::Starting => write!(f, "starting"),
            Self::Running => write!(f, "running"),
            Self::Complete => write!(f, "complete"),
            Self::Shutdown => write!(f, "shutdown"),
            Self::Failed => write!(f, "failed"),
            Self::Rejected => write!(f, "rejected"),
            Self::Remove => write!(f, "remove"),
            Self::Orphaned => write!(f, "orphaned"),
        }
    }
}
impl Default for TaskState {
    fn default() -> TaskState {
        Self::New
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TaskStatus : represents the status of a task.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TaskStatus {
    #[serde(rename = "Timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[serde(rename = "State", skip_serializing_if = "Option::is_none")]
    pub state: Option<models::TaskState>,
    #[serde(rename = "Message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Err", skip_serializing_if = "Option::is_none")]
    pub err: Option<String>,
    #[serde(rename = "ContainerStatus", skip_serializing_if = "Option::is_none")]
    pub container_status: Option<Box<models::ContainerStatus>>,
    #[serde(rename = "PortStatus", skip_serializing_if = "Option::is_none")]
    pub port_status: Option<Box<models::PortStatus>>,
}
impl TaskStatus {
    /// represents the status of a task.
    pub fn new() -> TaskStatus {
        TaskStatus {
            timestamp: None,
            state: None,
            message: None,
            err: None,
            container_status: None,
            port_status: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ThrottleDevice {
    /// Device path
    #[serde(rename = "Path", skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Rate
    #[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<u64>,
}
impl ThrottleDevice {
    pub fn new() -> ThrottleDevice {
        ThrottleDevice {
            path: None,
            rate: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// TlsInfo : Information about the issuer of leaf TLS certificates and the trusted root CA certificate.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TlsInfo {
    /// The root CA certificate(s) that are used to validate leaf TLS certificates.
    #[serde(rename = "TrustRoot", skip_serializing_if = "Option::is_none")]
    pub trust_root: Option<String>,
    /// The base64-url-safe-encoded raw subject bytes of the issuer.
    #[serde(rename = "CertIssuerSubject", skip_serializing_if = "Option::is_none")]
    pub cert_issuer_subject: Option<String>,
    /// The base64-url-safe-encoded raw public key bytes of the issuer.
    #[serde(rename = "CertIssuerPublicKey", skip_serializing_if = "Option::is_none")]
    pub cert_issuer_public_key: Option<String>,
}
impl TlsInfo {
    /// Information about the issuer of leaf TLS certificates and the trusted root CA certificate.
    pub fn new() -> TlsInfo {
        TlsInfo {
            trust_root: None,
            cert_issuer_subject: None,
            cert_issuer_public_key: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UnlockKeyResponse {
    /// The swarm's unlock key.
    #[serde(rename = "UnlockKey", skip_serializing_if = "Option::is_none")]
    pub unlock_key: Option<String>,
}
impl UnlockKeyResponse {
    pub fn new() -> UnlockKeyResponse {
        UnlockKeyResponse {
            unlock_key: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    /// Name of the volume.
    #[serde(rename = "Name")]
    pub name: String,
    /// Name of the volume driver used by the volume.
    #[serde(rename = "Driver")]
    pub driver: String,
    /// Mount path of the volume on the host.
    #[serde(rename = "Mountpoint")]
    pub mountpoint: String,
    /// Date/Time the volume was created.
    #[serde(rename = "CreatedAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Low-level details about the volume, provided by the volume driver. Details are returned as a map with key/value pairs: `{\"key\":\"value\",\"key2\":\"value2\"}`.  The `Status` field is optional, and is omitted if the volume driver does not support this feature.
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<std::collections::HashMap<String, serde_json::Value>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels")]
    pub labels: std::collections::HashMap<String, String>,
    /// The level at which the volume exists. Either `global` for cluster-wide, or `local` for machine level.
    #[serde(rename = "Scope")]
    pub scope: Scope,
    #[serde(rename = "ClusterVolume", skip_serializing_if = "Option::is_none")]
    pub cluster_volume: Option<Box<models::ClusterVolume>>,
    /// The driver specific options used when creating the volume.
    #[serde(rename = "Options")]
    pub options: std::collections::HashMap<String, String>,
    #[serde(
        rename = "UsageData",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub usage_data: Option<Option<Box<models::VolumeUsageData>>>,
}
impl Volume {
    pub fn new(
        name: String,
        driver: String,
        mountpoint: String,
        labels: std::collections::HashMap<String, String>,
        scope: Scope,
        options: std::collections::HashMap<String, String>,
    ) -> Volume {
        Volume {
            name,
            driver,
            mountpoint,
            created_at: None,
            status: None,
            labels,
            scope,
            cluster_volume: None,
            options,
            usage_data: None,
        }
    }
}
/// The level at which the volume exists. Either `global` for cluster-wide, or `local` for machine level.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize
)]
pub enum Scope {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "global")]
    Global,
}
impl Default for Scope {
    fn default() -> Scope {
        Self::Local
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// VolumeCreateOptions : Volume configuration
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeCreateOptions {
    /// The new volume's name. If not specified, Docker generates a name.
    #[serde(rename = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Name of the volume driver to use.
    #[serde(rename = "Driver", skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// A mapping of driver options and values. These options are passed directly to the driver and are driver specific.
    #[serde(rename = "DriverOpts", skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<std::collections::HashMap<String, String>>,
    /// User-defined key/value metadata.
    #[serde(rename = "Labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClusterVolumeSpec", skip_serializing_if = "Option::is_none")]
    pub cluster_volume_spec: Option<Box<models::ClusterVolumeSpec>>,
}
impl VolumeCreateOptions {
    /// Volume configuration
    pub fn new() -> VolumeCreateOptions {
        VolumeCreateOptions {
            name: None,
            driver: None,
            driver_opts: None,
            labels: None,
            cluster_volume_spec: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// VolumeListResponse : Volume list response
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeListResponse {
    /// List of volumes
    #[serde(rename = "Volumes", skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<models::Volume>>,
    /// Warnings that occurred when fetching the list of volumes.
    #[serde(rename = "Warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}
impl VolumeListResponse {
    /// Volume list response
    pub fn new() -> VolumeListResponse {
        VolumeListResponse {
            volumes: None,
            warnings: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumePruneResponse {
    /// Volumes that were deleted
    #[serde(rename = "VolumesDeleted", skip_serializing_if = "Option::is_none")]
    pub volumes_deleted: Option<Vec<String>>,
    /// Disk space reclaimed in bytes
    #[serde(rename = "SpaceReclaimed", skip_serializing_if = "Option::is_none")]
    pub space_reclaimed: Option<i64>,
}
impl VolumePruneResponse {
    pub fn new() -> VolumePruneResponse {
        VolumePruneResponse {
            volumes_deleted: None,
            space_reclaimed: None,
        }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// VolumeUpdateRequest : Volume configuration
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeUpdateRequest {
    #[serde(rename = "Spec", skip_serializing_if = "Option::is_none")]
    pub spec: Option<Box<models::ClusterVolumeSpec>>,
}
impl VolumeUpdateRequest {
    /// Volume configuration
    pub fn new() -> VolumeUpdateRequest {
        VolumeUpdateRequest { spec: None }
    }
}
use crate::models;
use serde::{Deserialize, Serialize};
/// VolumeUsageData : Usage details about the volume. This information is used by the `GET /system/df` endpoint, and omitted in other endpoints.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct VolumeUsageData {
    /// Amount of disk space used by the volume (in bytes). This information is only available for volumes created with the `\"local\"` volume driver. For volumes created with other volume drivers, this field is set to `-1` (\"not available\")
    #[serde(rename = "Size")]
    pub size: i64,
    /// The number of containers referencing this volume. This field is set to `-1` if the reference-count is not available.
    #[serde(rename = "RefCount")]
    pub ref_count: i64,
}
impl VolumeUsageData {
    /// Usage details about the volume. This information is used by the `GET /system/df` endpoint, and omitted in other endpoints.
    pub fn new(size: i64, ref_count: i64) -> VolumeUsageData {
        VolumeUsageData { size, ref_count }
    }
}
pub mod address;
pub use self::address::Address;
pub mod auth_config;
pub use self::auth_config::AuthConfig;
pub mod build_cache;
pub use self::build_cache::BuildCache;
pub mod build_info;
pub use self::build_info::BuildInfo;
pub mod build_prune_response;
pub use self::build_prune_response::BuildPruneResponse;
pub mod change_type;
pub use self::change_type::ChangeType;
pub mod cluster_info;
pub use self::cluster_info::ClusterInfo;
pub mod cluster_volume;
pub use self::cluster_volume::ClusterVolume;
pub mod cluster_volume_info;
pub use self::cluster_volume_info::ClusterVolumeInfo;
pub mod cluster_volume_publish_status_inner;
pub use self::cluster_volume_publish_status_inner::ClusterVolumePublishStatusInner;
pub mod cluster_volume_spec;
pub use self::cluster_volume_spec::ClusterVolumeSpec;
pub mod cluster_volume_spec_access_mode;
pub use self::cluster_volume_spec_access_mode::ClusterVolumeSpecAccessMode;
pub mod cluster_volume_spec_access_mode_accessibility_requirements;
pub use self::cluster_volume_spec_access_mode_accessibility_requirements::ClusterVolumeSpecAccessModeAccessibilityRequirements;
pub mod cluster_volume_spec_access_mode_capacity_range;
pub use self::cluster_volume_spec_access_mode_capacity_range::ClusterVolumeSpecAccessModeCapacityRange;
pub mod cluster_volume_spec_access_mode_secrets_inner;
pub use self::cluster_volume_spec_access_mode_secrets_inner::ClusterVolumeSpecAccessModeSecretsInner;
pub mod commit;
pub use self::commit::Commit;
pub mod config;
pub use self::config::Config;
pub mod config_create_request;
pub use self::config_create_request::ConfigCreateRequest;
pub mod config_reference;
pub use self::config_reference::ConfigReference;
pub mod config_spec;
pub use self::config_spec::ConfigSpec;
pub mod container_config;
pub use self::container_config::ContainerConfig;
pub mod container_create_request;
pub use self::container_create_request::ContainerCreateRequest;
pub mod container_create_response;
pub use self::container_create_response::ContainerCreateResponse;
pub mod container_inspect_response;
pub use self::container_inspect_response::ContainerInspectResponse;
pub mod container_prune_response;
pub use self::container_prune_response::ContainerPruneResponse;
pub mod container_state;
pub use self::container_state::ContainerState;
pub mod container_status;
pub use self::container_status::ContainerStatus;
pub mod container_summary;
pub use self::container_summary::ContainerSummary;
pub mod container_summary_host_config;
pub use self::container_summary_host_config::ContainerSummaryHostConfig;
pub mod container_summary_network_settings;
pub use self::container_summary_network_settings::ContainerSummaryNetworkSettings;
pub mod container_top_response;
pub use self::container_top_response::ContainerTopResponse;
pub mod container_update_request;
pub use self::container_update_request::ContainerUpdateRequest;
pub mod container_update_response;
pub use self::container_update_response::ContainerUpdateResponse;
pub mod container_wait_exit_error;
pub use self::container_wait_exit_error::ContainerWaitExitError;
pub mod container_wait_response;
pub use self::container_wait_response::ContainerWaitResponse;
pub mod containerd_info;
pub use self::containerd_info::ContainerdInfo;
pub mod containerd_info_namespaces;
pub use self::containerd_info_namespaces::ContainerdInfoNamespaces;
pub mod create_image_info;
pub use self::create_image_info::CreateImageInfo;
pub mod device_mapping;
pub use self::device_mapping::DeviceMapping;
pub mod device_request;
pub use self::device_request::DeviceRequest;
pub mod distribution_inspect;
pub use self::distribution_inspect::DistributionInspect;
pub mod driver;
pub use self::driver::Driver;
pub mod driver_data;
pub use self::driver_data::DriverData;
pub mod endpoint_ipam_config;
pub use self::endpoint_ipam_config::EndpointIpamConfig;
pub mod endpoint_port_config;
pub use self::endpoint_port_config::EndpointPortConfig;
pub mod endpoint_settings;
pub use self::endpoint_settings::EndpointSettings;
pub mod endpoint_spec;
pub use self::endpoint_spec::EndpointSpec;
pub mod engine_description;
pub use self::engine_description::EngineDescription;
pub mod engine_description_plugins_inner;
pub use self::engine_description_plugins_inner::EngineDescriptionPluginsInner;
pub mod error_detail;
pub use self::error_detail::ErrorDetail;
pub mod error_response;
pub use self::error_response::ErrorResponse;
pub mod event_actor;
pub use self::event_actor::EventActor;
pub mod event_message;
pub use self::event_message::EventMessage;
pub mod exec_config;
pub use self::exec_config::ExecConfig;
pub mod exec_inspect_response;
pub use self::exec_inspect_response::ExecInspectResponse;
pub mod exec_start_config;
pub use self::exec_start_config::ExecStartConfig;
pub mod filesystem_change;
pub use self::filesystem_change::FilesystemChange;
pub mod generic_resources_inner;
pub use self::generic_resources_inner::GenericResourcesInner;
pub mod generic_resources_inner_discrete_resource_spec;
pub use self::generic_resources_inner_discrete_resource_spec::GenericResourcesInnerDiscreteResourceSpec;
pub mod generic_resources_inner_named_resource_spec;
pub use self::generic_resources_inner_named_resource_spec::GenericResourcesInnerNamedResourceSpec;
pub mod health;
pub use self::health::Health;
pub mod health_config;
pub use self::health_config::HealthConfig;
pub mod healthcheck_result;
pub use self::healthcheck_result::HealthcheckResult;
pub mod history_response_item;
pub use self::history_response_item::HistoryResponseItem;
pub mod host_config;
pub use self::host_config::HostConfig;
pub mod host_config_all_of_log_config;
pub use self::host_config_all_of_log_config::HostConfigAllOfLogConfig;
pub mod id_response;
pub use self::id_response::IdResponse;
pub mod image_config;
pub use self::image_config::ImageConfig;
pub mod image_delete_response_item;
pub use self::image_delete_response_item::ImageDeleteResponseItem;
pub mod image_id;
pub use self::image_id::ImageId;
pub mod image_inspect;
pub use self::image_inspect::ImageInspect;
pub mod image_inspect_metadata;
pub use self::image_inspect_metadata::ImageInspectMetadata;
pub mod image_inspect_root_fs;
pub use self::image_inspect_root_fs::ImageInspectRootFs;
pub mod image_manifest_summary;
pub use self::image_manifest_summary::ImageManifestSummary;
pub mod image_manifest_summary_attestation_data;
pub use self::image_manifest_summary_attestation_data::ImageManifestSummaryAttestationData;
pub mod image_manifest_summary_image_data;
pub use self::image_manifest_summary_image_data::ImageManifestSummaryImageData;
pub mod image_manifest_summary_image_data_size;
pub use self::image_manifest_summary_image_data_size::ImageManifestSummaryImageDataSize;
pub mod image_manifest_summary_size;
pub use self::image_manifest_summary_size::ImageManifestSummarySize;
pub mod image_prune_response;
pub use self::image_prune_response::ImagePruneResponse;
pub mod image_search_response_item;
pub use self::image_search_response_item::ImageSearchResponseItem;
pub mod image_summary;
pub use self::image_summary::ImageSummary;
pub mod index_info;
pub use self::index_info::IndexInfo;
pub mod ipam;
pub use self::ipam::Ipam;
pub mod ipam_config;
pub use self::ipam_config::IpamConfig;
pub mod join_tokens;
pub use self::join_tokens::JoinTokens;
pub mod limit;
pub use self::limit::Limit;
pub mod local_node_state;
pub use self::local_node_state::LocalNodeState;
pub mod manager_status;
pub use self::manager_status::ManagerStatus;
pub mod mount;
pub use self::mount::Mount;
pub mod mount_bind_options;
pub use self::mount_bind_options::MountBindOptions;
pub mod mount_point;
pub use self::mount_point::MountPoint;
pub mod mount_tmpfs_options;
pub use self::mount_tmpfs_options::MountTmpfsOptions;
pub mod mount_volume_options;
pub use self::mount_volume_options::MountVolumeOptions;
pub mod mount_volume_options_driver_config;
pub use self::mount_volume_options_driver_config::MountVolumeOptionsDriverConfig;
pub mod network;
pub use self::network::Network;
pub mod network_attachment_config;
pub use self::network_attachment_config::NetworkAttachmentConfig;
pub mod network_connect_request;
pub use self::network_connect_request::NetworkConnectRequest;
pub mod network_container;
pub use self::network_container::NetworkContainer;
pub mod network_create_request;
pub use self::network_create_request::NetworkCreateRequest;
pub mod network_create_response;
pub use self::network_create_response::NetworkCreateResponse;
pub mod network_disconnect_request;
pub use self::network_disconnect_request::NetworkDisconnectRequest;
pub mod network_prune_response;
pub use self::network_prune_response::NetworkPruneResponse;
pub mod network_settings;
pub use self::network_settings::NetworkSettings;
pub mod networking_config;
pub use self::networking_config::NetworkingConfig;
pub mod node;
pub use self::node::Node;
pub mod node_description;
pub use self::node_description::NodeDescription;
pub mod node_spec;
pub use self::node_spec::NodeSpec;
pub mod node_state;
pub use self::node_state::NodeState;
pub mod node_status;
pub use self::node_status::NodeStatus;
pub mod object_version;
pub use self::object_version::ObjectVersion;
pub mod oci_descriptor;
pub use self::oci_descriptor::OciDescriptor;
pub mod oci_platform;
pub use self::oci_platform::OciPlatform;
pub mod peer_info;
pub use self::peer_info::PeerInfo;
pub mod peer_node;
pub use self::peer_node::PeerNode;
pub mod platform;
pub use self::platform::Platform;
pub mod plugin;
pub use self::plugin::Plugin;
pub mod plugin_config;
pub use self::plugin_config::PluginConfig;
pub mod plugin_config_args;
pub use self::plugin_config_args::PluginConfigArgs;
pub mod plugin_config_interface;
pub use self::plugin_config_interface::PluginConfigInterface;
pub mod plugin_config_linux;
pub use self::plugin_config_linux::PluginConfigLinux;
pub mod plugin_config_network;
pub use self::plugin_config_network::PluginConfigNetwork;
pub mod plugin_config_rootfs;
pub use self::plugin_config_rootfs::PluginConfigRootfs;
pub mod plugin_config_user;
pub use self::plugin_config_user::PluginConfigUser;
pub mod plugin_device;
pub use self::plugin_device::PluginDevice;
pub mod plugin_env;
pub use self::plugin_env::PluginEnv;
pub mod plugin_interface_type;
pub use self::plugin_interface_type::PluginInterfaceType;
pub mod plugin_mount;
pub use self::plugin_mount::PluginMount;
pub mod plugin_privilege;
pub use self::plugin_privilege::PluginPrivilege;
pub mod plugin_settings;
pub use self::plugin_settings::PluginSettings;
pub mod plugins_info;
pub use self::plugins_info::PluginsInfo;
pub mod port;
pub use self::port::Port;
pub mod port_binding;
pub use self::port_binding::PortBinding;
pub mod port_status;
pub use self::port_status::PortStatus;
pub mod process_config;
pub use self::process_config::ProcessConfig;
pub mod progress_detail;
pub use self::progress_detail::ProgressDetail;
pub mod push_image_info;
pub use self::push_image_info::PushImageInfo;
pub mod reachability;
pub use self::reachability::Reachability;
pub mod registry_service_config;
pub use self::registry_service_config::RegistryServiceConfig;
pub mod resource_object;
pub use self::resource_object::ResourceObject;
pub mod resources;
pub use self::resources::Resources;
pub mod resources_blkio_weight_device_inner;
pub use self::resources_blkio_weight_device_inner::ResourcesBlkioWeightDeviceInner;
pub mod resources_ulimits_inner;
pub use self::resources_ulimits_inner::ResourcesUlimitsInner;
pub mod restart_policy;
pub use self::restart_policy::RestartPolicy;
pub mod runtime;
pub use self::runtime::Runtime;
pub mod secret;
pub use self::secret::Secret;
pub mod secret_create_request;
pub use self::secret_create_request::SecretCreateRequest;
pub mod secret_spec;
pub use self::secret_spec::SecretSpec;
pub mod service;
pub use self::service::Service;
pub mod service_create_request;
pub use self::service_create_request::ServiceCreateRequest;
pub mod service_create_response;
pub use self::service_create_response::ServiceCreateResponse;
pub mod service_endpoint;
pub use self::service_endpoint::ServiceEndpoint;
pub mod service_endpoint_virtual_ips_inner;
pub use self::service_endpoint_virtual_ips_inner::ServiceEndpointVirtualIpsInner;
pub mod service_job_status;
pub use self::service_job_status::ServiceJobStatus;
pub mod service_service_status;
pub use self::service_service_status::ServiceServiceStatus;
pub mod service_spec;
pub use self::service_spec::ServiceSpec;
pub mod service_spec_mode;
pub use self::service_spec_mode::ServiceSpecMode;
pub mod service_spec_mode_replicated;
pub use self::service_spec_mode_replicated::ServiceSpecModeReplicated;
pub mod service_spec_mode_replicated_job;
pub use self::service_spec_mode_replicated_job::ServiceSpecModeReplicatedJob;
pub mod service_spec_rollback_config;
pub use self::service_spec_rollback_config::ServiceSpecRollbackConfig;
pub mod service_spec_update_config;
pub use self::service_spec_update_config::ServiceSpecUpdateConfig;
pub mod service_update_request;
pub use self::service_update_request::ServiceUpdateRequest;
pub mod service_update_response;
pub use self::service_update_response::ServiceUpdateResponse;
pub mod service_update_status;
pub use self::service_update_status::ServiceUpdateStatus;
pub mod swarm;
pub use self::swarm::Swarm;
pub mod swarm_info;
pub use self::swarm_info::SwarmInfo;
pub mod swarm_init_request;
pub use self::swarm_init_request::SwarmInitRequest;
pub mod swarm_join_request;
pub use self::swarm_join_request::SwarmJoinRequest;
pub mod swarm_spec;
pub use self::swarm_spec::SwarmSpec;
pub mod swarm_spec_ca_config;
pub use self::swarm_spec_ca_config::SwarmSpecCaConfig;
pub mod swarm_spec_ca_config_external_cas_inner;
pub use self::swarm_spec_ca_config_external_cas_inner::SwarmSpecCaConfigExternalCasInner;
pub mod swarm_spec_dispatcher;
pub use self::swarm_spec_dispatcher::SwarmSpecDispatcher;
pub mod swarm_spec_encryption_config;
pub use self::swarm_spec_encryption_config::SwarmSpecEncryptionConfig;
pub mod swarm_spec_orchestration;
pub use self::swarm_spec_orchestration::SwarmSpecOrchestration;
pub mod swarm_spec_raft;
pub use self::swarm_spec_raft::SwarmSpecRaft;
pub mod swarm_spec_task_defaults;
pub use self::swarm_spec_task_defaults::SwarmSpecTaskDefaults;
pub mod swarm_spec_task_defaults_log_driver;
pub use self::swarm_spec_task_defaults_log_driver::SwarmSpecTaskDefaultsLogDriver;
pub mod swarm_unlock_request;
pub use self::swarm_unlock_request::SwarmUnlockRequest;
pub mod system_auth_response;
pub use self::system_auth_response::SystemAuthResponse;
pub mod system_data_usage_response;
pub use self::system_data_usage_response::SystemDataUsageResponse;
pub mod system_info;
pub use self::system_info::SystemInfo;
pub mod system_info_default_address_pools_inner;
pub use self::system_info_default_address_pools_inner::SystemInfoDefaultAddressPoolsInner;
pub mod system_version;
pub use self::system_version::SystemVersion;
pub mod system_version_components_inner;
pub use self::system_version_components_inner::SystemVersionComponentsInner;
pub mod system_version_platform;
pub use self::system_version_platform::SystemVersionPlatform;
pub mod task;
pub use self::task::Task;
pub mod task_spec;
pub use self::task_spec::TaskSpec;
pub mod task_spec_container_spec;
pub use self::task_spec_container_spec::TaskSpecContainerSpec;
pub mod task_spec_container_spec_configs_inner;
pub use self::task_spec_container_spec_configs_inner::TaskSpecContainerSpecConfigsInner;
pub mod task_spec_container_spec_configs_inner_file;
pub use self::task_spec_container_spec_configs_inner_file::TaskSpecContainerSpecConfigsInnerFile;
pub mod task_spec_container_spec_dns_config;
pub use self::task_spec_container_spec_dns_config::TaskSpecContainerSpecDnsConfig;
pub mod task_spec_container_spec_privileges;
pub use self::task_spec_container_spec_privileges::TaskSpecContainerSpecPrivileges;
pub mod task_spec_container_spec_privileges_app_armor;
pub use self::task_spec_container_spec_privileges_app_armor::TaskSpecContainerSpecPrivilegesAppArmor;
pub mod task_spec_container_spec_privileges_credential_spec;
pub use self::task_spec_container_spec_privileges_credential_spec::TaskSpecContainerSpecPrivilegesCredentialSpec;
pub mod task_spec_container_spec_privileges_se_linux_context;
pub use self::task_spec_container_spec_privileges_se_linux_context::TaskSpecContainerSpecPrivilegesSeLinuxContext;
pub mod task_spec_container_spec_privileges_seccomp;
pub use self::task_spec_container_spec_privileges_seccomp::TaskSpecContainerSpecPrivilegesSeccomp;
pub mod task_spec_container_spec_secrets_inner;
pub use self::task_spec_container_spec_secrets_inner::TaskSpecContainerSpecSecretsInner;
pub mod task_spec_container_spec_secrets_inner_file;
pub use self::task_spec_container_spec_secrets_inner_file::TaskSpecContainerSpecSecretsInnerFile;
pub mod task_spec_log_driver;
pub use self::task_spec_log_driver::TaskSpecLogDriver;
pub mod task_spec_network_attachment_spec;
pub use self::task_spec_network_attachment_spec::TaskSpecNetworkAttachmentSpec;
pub mod task_spec_placement;
pub use self::task_spec_placement::TaskSpecPlacement;
pub mod task_spec_placement_preferences_inner;
pub use self::task_spec_placement_preferences_inner::TaskSpecPlacementPreferencesInner;
pub mod task_spec_placement_preferences_inner_spread;
pub use self::task_spec_placement_preferences_inner_spread::TaskSpecPlacementPreferencesInnerSpread;
pub mod task_spec_plugin_spec;
pub use self::task_spec_plugin_spec::TaskSpecPluginSpec;
pub mod task_spec_resources;
pub use self::task_spec_resources::TaskSpecResources;
pub mod task_spec_restart_policy;
pub use self::task_spec_restart_policy::TaskSpecRestartPolicy;
pub mod task_state;
pub use self::task_state::TaskState;
pub mod task_status;
pub use self::task_status::TaskStatus;
pub mod throttle_device;
pub use self::throttle_device::ThrottleDevice;
pub mod tls_info;
pub use self::tls_info::TlsInfo;
pub mod unlock_key_response;
pub use self::unlock_key_response::UnlockKeyResponse;
pub mod volume;
pub use self::volume::Volume;
pub mod volume_create_options;
pub use self::volume_create_options::VolumeCreateOptions;
pub mod volume_list_response;
pub use self::volume_list_response::VolumeListResponse;
pub mod volume_prune_response;
pub use self::volume_prune_response::VolumePruneResponse;
pub mod volume_update_request;
pub use self::volume_update_request::VolumeUpdateRequest;
pub mod volume_usage_data;
pub use self::volume_usage_data::VolumeUsageData;
