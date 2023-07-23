use std::collections::HashMap;
use std::path::PathBuf;

use vizia::prelude::*;
use serde::Serialize;
use serde::Deserialize;

// TODO: replace with u128 when RON supports it

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Command {
    pub cmd: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub env: Option<HashMap<String, String>>
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum GitOrigin {
    #[default]
    Head,
    Branch(String),
    Commit(String)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeSource {
    Static(String),
    Prebuilt(String),
    Cargo {
        repo: String,
        origin: Option<GitOrigin>
    },
    Custom(Command)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FSNode {
    pub path: PathBuf,
    pub source: NodeSource,
    pub dependencies: Option<Vec<PathBuf>>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PartitionPreset {
    Ext4 {
        filesystem: Vec<FSNode>,
        post_run: Option<Command>
    },
    Fat32 {
        filesystem: Vec<FSNode>,
        post_run: Option<Command>
    },
    RedoxFS {
        filesystem: Vec<FSNode>,
        post_run: Option<Command>
    },
    Boot {
        post_run: Command
    },
    Swap,
    Raw
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Partition {
    pub label: String,
    pub purpose: PartitionPreset,
    pub offset: Option<u64>,
    pub size: Option<u64>
}

#[derive(Lens, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BuildConfig {
    pub name: String,
    pub partitions: Vec<Partition>,
    pub image_size: Option<u64>
}

impl Data for BuildConfig {
    fn same(&self, other: &Self) -> bool {
        *self == *other
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    
    #[test]
    pub fn example_config() {
        let config = build::BuildConfig {
            name: "Test Config".to_owned(),
            partitions: vec![build::Partition {
                label: "boot".to_owned(),
                offset: Some(34),
                size: None,
                purpose: build::PartitionPreset::Boot {
                    post_run: build::Command {
                        cmd: vec!["/usr/bin/nu".to_owned(), "-c".to_owned(), "echo hello".to_owned()],
                        cwd: None,
                        env: None
                    }
                },
            }],
            image_size: Some(268435456),
        };
        
        println!("{:?}", &config);
        if let Ok(config) = ron::ser::to_string_pretty(&config, ron::ser::PrettyConfig::new()
            .indentor("    ".to_owned()))  {
                println!("{}", config);
            }
            
    }
}