use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use glob::glob;

use regex::{Captures, Regex};

struct JunosVersion {
    major: usize,
    minor: usize,
}

impl JunosVersion {
    fn from_captures(caps: Captures<'_>) -> Option<Self> {
        let get = |name| caps.name(name).and_then(|m| m.as_str().parse().ok());
        let (major, minor) = (get("major")?, get("minor")?);
        Some(Self { major, minor })
    }

    fn paths(&self, suffix: &str) -> Result<Vec<PathBuf>, Box<dyn Error>> {
        let pattern = format!("protos/{}.{}/**/2/{suffix}", self.major, self.minor);
        Ok(glob(&pattern)?.collect::<Result<_, _>>()?)
    }

    fn out_dir(&self) -> Result<PathBuf, Box<dyn Error>> {
        let path = env::var("OUT_DIR")
            .map(PathBuf::from)
            .map(|base| base.join(format!("junos_{}_{}", self.major, self.minor)))?;
        fs::create_dir(&path)?;
        Ok(path)
    }

    fn compile_protos(&self) -> Result<(), Box<dyn Error>> {
        // create the version specific output directory
        let out_dir = self.out_dir()?;

        // construct the paths to the `*.proto` files and includes directory
        let includes = self.paths("")?;
        let protos = self.paths("*.proto")?;

        // compile the protobuf definitions into rust source code
        tonic_build::configure()
            .build_server(false)
            .out_dir(out_dir)
            .include_file("jnx.jet.rs")
            .compile(&protos, &includes)?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // build a `Vec` of requested `junos-*` features
    let feature_re = Regex::new(r"^CARGO_FEATURE_JUNOS_(?P<major>\d+)_(?P<minor>\d+)$")?;
    let versions = env::vars()
        .filter_map(|(key, _)| {
            feature_re
                .captures(&key)
                .and_then(JunosVersion::from_captures)
        })
        .collect::<Vec<_>>();

    // check that we have at least one "version" feature enabled
    if versions.is_empty() {
        return Err("At least one `junos-X-Y` feature must be enabled".into());
    }

    // compile the versioned protobuf definitions
    versions.iter().try_for_each(JunosVersion::compile_protos)
}
