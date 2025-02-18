use std::sync::Arc;
use reqwest::blocking::ClientBuilder;
use crate::api::internals::api_sync::{ConfigApi, ContainerApi, DistributionApi, ExecApi, ImageApi, NetworkApi, NodeApi, PluginApi, SecretApi, ServiceApi, SessionApi, SwarmApi, SystemApi, TaskApi, VolumeApi};
use crate::api::internals::api_sync::Configuration;

pub struct ContainerClientBuilder {
    base_path: Option<String>,
    user_agent: Option<String>,
    http1_only: Option<bool>,
    https_only: Option<bool>,
}

impl ContainerClientBuilder {
    pub fn with_base_path(&mut self, base_path: String) -> &mut Self {
        self.base_path = Some(base_path);
        self
    }

    pub fn with_user_agent(&mut self, user_agent: String) -> &mut Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn http1_only(&mut self, http1_only: bool) -> &mut Self {
        self.http1_only = Some(http1_only);
        self
    }

    pub fn with_https_only(&mut self, https_only: bool) -> &mut Self {
        self.https_only = Some(https_only);
        self
    }

    pub fn build(&self) -> ContainerClient {
        if self.base_path.is_none() {
            panic!("Base path should be provided.")
        }
        if self.user_agent.is_none() {
            panic!("User agent should be provided.")
        }
        let mut client_builder = ClientBuilder::new();
        if let Some(_) = self.http1_only {
            client_builder = client_builder.http1_only();
        }
        if let Some(_) = self.https_only {
            client_builder = client_builder.https_only(true);
        }
        let configuration = Configuration {
            base_path: self.base_path.clone().unwrap(),
            user_agent: self.user_agent.clone(),
            client: client_builder
                .build()
                .unwrap(),
        };
        ContainerClient::new(configuration)
    }
}

pub struct ContainerClient {
    pub config: ConfigApi,
    pub container: ContainerApi,
    pub distribution: DistributionApi,
    pub exec: ExecApi,
    pub image: ImageApi,
    pub network: NetworkApi,
    pub node: NodeApi,
    pub plugin: PluginApi,
    pub secret: SecretApi,
    pub service: ServiceApi,
    pub session: SessionApi,
    pub swarm: SwarmApi,
    pub system: SystemApi,
    pub task: TaskApi,
    pub volume: VolumeApi,
}

impl ContainerClient {
    pub(self) fn new(configuration: Configuration) -> Self {
        let configuration = Arc::new(configuration);
        Self {
            config: ConfigApi::new(configuration.clone()),
            container: ContainerApi::new(configuration.clone()),
            distribution: DistributionApi::new(configuration.clone()),
            exec: ExecApi::new(configuration.clone()),
            image: ImageApi::new(configuration.clone()),
            network: NetworkApi::new(configuration.clone()),
            node: NodeApi::new(configuration.clone()),
            plugin: PluginApi::new(configuration.clone()),
            secret: SecretApi::new(configuration.clone()),
            service: ServiceApi::new(configuration.clone()),
            session: SessionApi::new(configuration.clone()),
            swarm: SwarmApi::new(configuration.clone()),
            system: SystemApi::new(configuration.clone()),
            task: TaskApi::new(configuration.clone()),
            volume: VolumeApi::new(configuration.clone()),
        }
    }
}

impl Default for ContainerClient {
    fn default() -> Self {
        let configuration = Configuration::default();
        Self::new(configuration)
    }
}