//! <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/generate-package-manifest>

use anyhow::Result;
use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/schema-root>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(
    rename = "Package",
    deny_unknown_fields,
    rename_all(serialize = "PascalCase")
)]
pub struct AppxManifest {
    #[serde(rename(serialize = "xmlns"))]
    #[serde(default = "default_namespace")]
    ns: String,
    #[serde(rename(serialize = "xmlns:uap"))]
    #[serde(default = "default_uap_namespace")]
    ns_uap: String,
    #[serde(rename(serialize = "xmlns:rescap"))]
    #[serde(default = "default_rescap_namespace")]
    ns_rescap: String,
    pub identity: Identity,
    pub properties: Properties,
    pub resources: Resources,
    pub dependencies: Dependencies,
    #[serde(serialize_with = "serialize_element")]
    pub capabilities: Vec<Capability>,
    pub applications: Applications,
}

impl Default for AppxManifest {
    fn default() -> Self {
        Self {
            ns: default_namespace(),
            ns_uap: default_uap_namespace(),
            ns_rescap: default_rescap_namespace(),
            identity: Default::default(),
            properties: Default::default(),
            resources: Default::default(),
            dependencies: Default::default(),
            capabilities: Default::default(),
            applications: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Applications {
    pub application: Vec<Application>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Resources {
    pub resource: Vec<Resource>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Dependencies {
    pub target_device_family: Vec<TargetDeviceFamily>,
}
/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-identity>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Identity {
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub processor_architecture: Option<String>,
    // pub processor_architecture: ResourceId<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Properties {
    #[serde(serialize_with = "serialize_element")]
    pub display_name: Option<String>,
    #[serde(serialize_with = "serialize_element")]
    pub publisher_display_name: Option<String>,
    #[serde(serialize_with = "serialize_element")]
    pub logo: Option<String>,
    #[serde(serialize_with = "serialize_element")]
    pub description: Option<String>,
}

fn serialize_element<S>(value: &impl Serialize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut tuple = serializer.serialize_tuple(1)?;
    tuple.serialize_element(value)?;
    tuple.end()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Resource {
    pub language: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct TargetDeviceFamily {
    pub name: String,
    pub min_version: String,
    pub max_version_tested: String,
}

impl Default for TargetDeviceFamily {
    fn default() -> Self {
        Self {
            name: "Windows.Desktop".into(),
            min_version: "10.0.0.0".into(),
            max_version_tested: "10.0.20348.0".into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub enum Capability {
    #[serde(rename(deserialize = "capability"))]
    #[serde(rename(serialize = "Capability"))]
    Capability {
        #[serde(rename(serialize = "Name"))]
        name: String,
    },
    #[serde(rename(deserialize = "restricted"))]
    #[serde(rename(serialize = "rescap:Capability"))]
    Restricted {
        #[serde(rename(serialize = "Name"))]
        name: String,
    },
    #[serde(rename(deserialize = "device"))]
    #[serde(rename(serialize = "DeviceCapability"))]
    Device {
        #[serde(rename(serialize = "Name"))]
        name: String,
    },
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Application {
    pub id: Option<String>,
    pub executable: Option<String>,
    pub entry_point: Option<String>,
    #[serde(rename(serialize = "uap:VisualElements"))]
    pub visual_elements: VisualElements,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct VisualElements {
    pub background_color: Option<String>,
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[serde(rename(serialize = "Square150x150Logo"))]
    pub logo_150x150: Option<String>,
    #[serde(rename(serialize = "Square44x44Logo"))]
    pub logo_44x44: Option<String>,
    #[serde(rename(serialize = "uap:DefaultTile"))]
    pub default_tile: Option<DefaultTile>,
    #[serde(rename(serialize = "uap:SplashScreen"))]
    pub splash_screen: Option<SplashScreen>,
    #[serde(rename(serialize = "uap:LockScreen"))]
    pub lock_screen: Option<LockScreen>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct DefaultTile {
    pub short_name: Option<String>,
    #[serde(rename(serialize = "Square71x71Logo"))]
    pub logo_71x71: Option<String>,
    #[serde(rename(serialize = "Square310x310Logo"))]
    pub logo_310x310: Option<String>,
    #[serde(rename(serialize = "Wide310x150Logo"))]
    pub logo_310x150: Option<String>,
    #[serde(rename(serialize = "uap:ShowNameOnTiles"))]
    pub show_names_on_tiles: ShowNameOnTiles,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShowNameOnTiles {
    #[serde(rename(serialize = "uap:ShowOn"))]
    pub show_on: Vec<ShowOn>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct ShowOn {
    pub tile: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct SplashScreen {
    pub image: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct LockScreen {
    pub badge_logo: String,
    pub notification: String,
}

fn default_namespace() -> String {
    "http://schemas.microsoft.com/appx/manifest/foundation/windows10".to_string()
}

fn default_uap_namespace() -> String {
    "http://schemas.microsoft.com/appx/manifest/uap/windows10".to_string()
}

fn default_rescap_namespace() -> String {
    "http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties() {
        let props = Properties {
            display_name: Some("".into()),
            publisher_display_name: Some("".into()),
            logo: Some("".into()),
            description: Some("".into()),
        };
        let xml = quick_xml::se::to_string(&props).unwrap();
        assert_eq!(xml, "<Properties><DisplayName></DisplayName><PublisherDisplayName></PublisherDisplayName><Logo></Logo><Description></Description></Properties>");
    }

    #[test]
    fn test_manifest() {
        let manifest = AppxManifest {
            identity: Identity {
                name: "com.flutter.fluttertodoapp".into(),
                version: "1.0.0.0".into(),
                publisher: "CN=Msix Testing, O=Msix Testing Corporation, S=Some-State, C=US".into(),
                processor_architecture: Some("x64".into()),
            },
            properties: Properties {
                display_name: Some("fluttertodoapp".into()),
                publisher_display_name: Some("com.flutter.fluttertodoapp".into()),
                logo: Some("Images\\StoreLogo.png".into()),
                description: Some("A new Flutter project.".into()),
            },
            resources: Resources {
                resource: vec![Resource {
                    language: "en".into(),
                }],
            },
            dependencies: Dependencies {
                target_device_family: vec![Default::default()],
            },
            capabilities: vec![
                Capability::Capability {
                    name: "internetClient".into(),
                },
                Capability::Restricted {
                    name: "runFullTrust".into(),
                },
                Capability::Device {
                    name: "location".into(),
                },
            ],
            applications: Applications {
                application: vec![Application {
                    id: Some("fluttertodoapp".into()),
                    executable: Some("todoapp.exe".into()),
                    entry_point: Some("Windows.FullTrustApplication".into()),
                    visual_elements: VisualElements {
                        background_color: Some("transparent".into()),
                        display_name: Some("fluttertodoapp".into()),
                        description: Some("A new flutter project.".into()),
                        logo_44x44: Some("Images\\Square44x44Logo.png".into()),
                        logo_150x150: Some("Images\\Square150x150Logo.png".into()),
                        default_tile: Some(DefaultTile {
                            short_name: Some("fluttertodoapp".into()),
                            logo_71x71: Some("Images\\SmallTile.png".into()),
                            logo_310x310: Some("Images\\LargeTile.png".into()),
                            logo_310x150: Some("Images\\Wide310x150Logo.png".into()),
                            show_names_on_tiles: ShowNameOnTiles {
                                show_on: vec![
                                    ShowOn {
                                        tile: "square150x150Logo".into(),
                                    },
                                    ShowOn {
                                        tile: "square310x310Logo".into(),
                                    },
                                    ShowOn {
                                        tile: "wide310x150Logo".into(),
                                    },
                                ],
                            },
                        }),
                        splash_screen: Some(SplashScreen {
                            image: "Images\\SplashScreen.png".into(),
                        }),
                        lock_screen: Some(LockScreen {
                            badge_logo: "Images\\BadgeLogo.png".into(),
                            notification: "badge".into(),
                        }),
                    },
                }],
            },
            ..Default::default()
        };
        let xml = quick_xml::se::to_string(&manifest).unwrap();
        println!("{}", xml);
    }
}
