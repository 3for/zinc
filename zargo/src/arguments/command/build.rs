//!
//! The Zargo package manager `build` subcommand.
//!

use std::convert::TryFrom;
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use crate::error::Error;
use crate::executable::compiler::Compiler;
use crate::http::downloader::Downloader;
use crate::http::Client as HttpClient;
use crate::network::Network;
use crate::project::data::private_key::PrivateKey as PrivateKeyFile;
use crate::project::data::Directory as DataDirectory;
use crate::project::target::deps::Directory as TargetDependenciesDirectory;
use crate::project::target::Directory as TargetDirectory;

///
/// The Zargo package manager `build` subcommand.
///
#[derive(Debug, StructOpt)]
#[structopt(about = "Builds the project at the given path")]
pub struct Command {
    /// Prints more logs, if passed several times.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    pub verbosity: usize,

    /// The path to the Zinc project manifest file.
    #[structopt(
        long = "manifest-path",
        parse(from_os_str),
        default_value = "./Zargo.toml"
    )]
    pub manifest_path: PathBuf,

    /// Builds the release version.
    #[structopt(long = "release")]
    pub is_release: bool,

    /// Sets the network name, where the contract must be published to.
    #[structopt(long = "network", default_value = "localhost")]
    pub network: String,
}

impl Command {
    ///
    /// Executes the command.
    ///
    pub async fn execute(self) -> anyhow::Result<()> {
        let manifest = zinc_manifest::Manifest::try_from(&self.manifest_path)?;

        let mut manifest_path = self.manifest_path.clone();
        if manifest_path.is_file() {
            manifest_path.pop();
        }

        if let zinc_manifest::ProjectType::Contract = manifest.project.r#type {
            if !PrivateKeyFile::exists_at(&manifest_path) {
                PrivateKeyFile::default().write_to(&manifest_path)?;
            }
        }

        TargetDirectory::create(&manifest_path, self.is_release)?;

        TargetDependenciesDirectory::create(&manifest_path)?;
        let target_deps_directory_path = TargetDependenciesDirectory::path(&manifest_path);

        DataDirectory::create(&manifest_path)?;

        if let Some(dependencies) = manifest.dependencies {
            let network = zksync::Network::from_str(self.network.as_str())
                .map(Network::from)
                .map_err(Error::NetworkInvalid)?;
            let url = network
                .try_into_url()
                .map_err(Error::NetworkUnimplemented)?;
            let http_client = HttpClient::new(url);
            let mut downloader = Downloader::new(&http_client, target_deps_directory_path);
            downloader.download_list(dependencies).await?;
        }

        if self.is_release {
            Compiler::build_release(
                self.verbosity,
                manifest.project.name.as_str(),
                &manifest.project.version,
                &manifest_path,
                false,
            )?;
        } else {
            Compiler::build_debug(
                self.verbosity,
                manifest.project.name.as_str(),
                &manifest.project.version,
                &manifest_path,
                false,
            )?;
        }

        Ok(())
    }
}
