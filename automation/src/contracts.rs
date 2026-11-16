use crate::{
    error::{Error, Result},
    json,
};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub(crate) struct Contracts {
    catalog: jsonschema::Validator,
    descriptor: jsonschema::Validator,
    application: jsonschema::Validator,
}

#[derive(Deserialize)]
pub(crate) struct Catalog {
    pub(crate) organization: String,
    pub(crate) product: String,
    pub(crate) components: Vec<Component>,
}
#[derive(Deserialize)]
pub(crate) struct Component {
    pub(crate) id: String,
    pub(crate) repository: String,
    pub(crate) kind: String,
    pub(crate) platforms: Vec<String>,
    pub(crate) path: String,
}

fn invalid(reason: &'static str) -> Error {
    Error::Contract(reason)
}

impl Contracts {
    pub(crate) fn open(root: &Path) -> Result<Self> {
        fn schema(root: &Path, name: &str) -> Result<jsonschema::Validator> {
            let value = json::load(&root.join("contracts").join(name))?;
            jsonschema::options()
                .with_draft(jsonschema::Draft::Draft202012)
                .should_validate_formats(false)
                .build(&value)
                .map_err(|_| invalid("invalid owned JSON schema"))
        }
        Ok(Self {
            catalog: schema(root, "module-catalog.schema.json")?,
            descriptor: schema(root, "module-manifest.json")?,
            application: schema(root, "app-release.schema.json")?,
        })
    }
    pub(crate) fn catalog(&self, value: Value) -> Result<Catalog> {
        self.catalog
            .validate(&value)
            .map_err(|_| invalid("invalid catalog shape"))?;
        let catalog: Catalog =
            serde_json::from_value(value).map_err(|_| invalid("invalid catalog fields"))?;
        let mut ids = BTreeSet::new();
        let mut repositories = BTreeSet::new();
        for item in &catalog.components {
            if !ids.insert(&item.id) || !repositories.insert(&item.repository) {
                return Err(invalid("duplicate catalog identity or repository"));
            }
            if item.platforms.iter().any(|p| p == "all") && item.platforms.len() != 1 {
                return Err(invalid("all cannot be combined with named platforms"));
            }
            let expected = match item.id.as_str() {
                "design-system" => "nddev-opennetwork-design-system".to_owned(),
                "organization" => ".github".to_owned(),
                id => format!("nddev-device-sync-{id}"),
            };
            if item.repository != expected {
                return Err(invalid("repository differs from its component boundary"));
            }
            if (item.id == "organization") != (item.kind == "organization-profile") {
                return Err(invalid(
                    "organization profile has a different ownership boundary",
                ));
            }
            let namespace = if item.kind == "tool" {
                "tools"
            } else {
                "modules"
            };
            if item.path != format!("{namespace}/{}", item.id) {
                return Err(invalid(
                    "component path differs from its type or privacy boundary",
                ));
            }
        }
        Ok(catalog)
    }
    pub(crate) fn descriptor(&self, value: Value) -> Result<()> {
        self.descriptor
            .validate(&value)
            .map_err(|_| invalid("invalid runtime descriptor"))?;
        #[derive(Deserialize)]
        struct Descriptor {
            id: String,
            dependencies: Vec<Dependency>,
        }
        #[derive(Deserialize)]
        struct Dependency {
            id: String,
        }
        let descriptor: Descriptor = serde_json::from_value(value)
            .map_err(|_| invalid("invalid runtime descriptor fields"))?;
        let mut ids = BTreeSet::new();
        for dependency in descriptor.dependencies {
            if dependency.id == descriptor.id || !ids.insert(dependency.id) {
                return Err(invalid("self or duplicate module dependency"));
            }
        }
        Ok(())
    }
    pub(crate) fn application(
        &self,
        value: Value,
        catalog: &Catalog,
        development_http: bool,
    ) -> Result<()> {
        self.application
            .validate(&value)
            .map_err(|_| invalid("invalid application candidate"))?;
        #[derive(Deserialize)]
        struct Candidate {
            version: String,
            sources: Vec<Source>,
            artifact: Artifact,
        }
        #[derive(Deserialize)]
        struct Source {
            role: String,
            repository: String,
            commit: String,
        }
        #[derive(Deserialize)]
        struct Artifact {
            url: String,
            app_source: String,
            native_source: String,
        }
        let candidate: Candidate =
            serde_json::from_value(value).map_err(|_| invalid("invalid application fields"))?;
        let number = candidate
            .version
            .strip_prefix("1.0.0-alpha.")
            .and_then(|v| v.parse::<u32>().ok())
            .filter(|v| *v > 0);
        if number.is_none() {
            return Err(invalid(
                "application candidate exceeds its unsigned 32-bit bound",
            ));
        }
        let mut allowed: BTreeSet<_> = catalog
            .components
            .iter()
            .map(|c| {
                (
                    catalog.organization.to_lowercase(),
                    c.repository.to_lowercase(),
                )
            })
            .collect();
        allowed.insert((
            catalog.organization.to_lowercase(),
            catalog.product.to_lowercase(),
        ));
        let mut sources = BTreeMap::new();
        for source in &candidate.sources {
            if sources.insert(source.role.as_str(), source).is_some() {
                return Err(invalid("duplicate application source role"));
            }
            let (owner, name) = source
                .repository
                .strip_prefix("https://github.com/")
                .and_then(|v| v.split_once('/'))
                .ok_or_else(|| invalid("invalid source repository"))?;
            let name = name.to_lowercase();
            if !allowed.contains(&(
                owner.to_lowercase(),
                name.strip_suffix(".git").unwrap_or(&name).to_owned(),
            )) {
                return Err(invalid("application source is outside the public assembly"));
            }
        }
        let desktop = catalog
            .components
            .iter()
            .find(|c| c.id == "desktop")
            .ok_or_else(|| invalid("candidate has no Desktop source boundary"))?;
        let expected = format!(
            "https://github.com/{}/{}",
            catalog.organization, desktop.repository
        );
        let desktop = sources
            .get("desktop")
            .ok_or_else(|| invalid("candidate has no Desktop source"))?;
        let artifact = candidate.artifact;
        if desktop.repository != expected
            || desktop.commit != artifact.app_source
            || artifact.native_source != artifact.app_source
        {
            return Err(invalid(
                "wrapper and native sources must match their Desktop source",
            ));
        }
        let url = url::Url::parse(&artifact.url).map_err(|_| invalid("invalid artifact URL"))?;
        let authority = artifact
            .url
            .split_once("://")
            .map(|v| v.1.split(['/', '?', '#']).next().unwrap_or(""))
            .unwrap_or("");
        if authority.is_empty()
            || authority.contains(['@', '\\'])
            || url.host_str().is_none()
            || url.fragment().is_some()
        {
            return Err(invalid(
                "artifact URL contains invalid authority, credentials or a fragment",
            ));
        }
        if url.scheme() != "https" {
            let port = url
                .port()
                .filter(|p| *p >= 1024)
                .ok_or_else(|| invalid("HTTP requires an explicit development loopback port"))?;
            if !development_http
                || url.scheme() != "http"
                || authority != format!("127.0.0.1:{port}")
            {
                return Err(invalid(
                    "HTTP is restricted to explicit literal development loopback",
                ));
            }
        }
        Ok(())
    }
}
