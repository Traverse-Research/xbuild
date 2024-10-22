//! <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/generate-package-manifest>
//!
//! <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/schema-root>
use anyhow::Result;
use serde::ser::{SerializeTuple, Serializer};
use serde::{Deserialize, Serialize};

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-package>
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
    #[serde(default)] // Not optional, but the only nested array may be empty
    pub resources: Resources,
    #[serde(default)] // Required, but default is provided by xbuild
    pub dependencies: Dependencies,
    #[serde(default, serialize_with = "serialize_element")]
    pub capabilities: Vec<Capability>,
    #[serde(default)] // Required, but default is provided by xbuild
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

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-applications>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Applications {
    /// 1-100 elements
    pub application: Vec<Application>,
}

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-resources>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Resources {
    /// 0 - 200 elements
    pub resource: Vec<Resource>,
}

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-dependencies>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Dependencies {
    /// 1 - 128 elements
    pub target_device_family: Vec<TargetDeviceFamily>,
    // pub package_dependency: Vec<>,
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

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-properties>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Properties {
    #[serde(serialize_with = "serialize_element")]
    pub display_name: String,
    #[serde(serialize_with = "serialize_element")]
    pub publisher_display_name: String,
    #[serde(serialize_with = "serialize_element")]
    pub logo: String,
    #[serde(default, serialize_with = "serialize_element")]
    pub description: Option<String>,
}

fn serialize_element<S>(value: &impl Serialize, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut tuple = serializer.serialize_tuple(1)?;
    // TODO: Skip if None!
    tuple.serialize_element(value)?;
    tuple.end()
}

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-resource>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Resource {
    pub language: Option<String>,
    #[serde(rename(serialize = "uap:Scale"))]
    pub scale: Option<u32>,
    #[serde(rename(serialize = "uap:DXFeatureLevel"))]
    pub dx_feature_level: Option<String>,
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
            // Add-AppxPackage : Deployment failed with HRESULT: 0x80080204, The Appx package's manifest is invalid.                   error 0x80080204: App manifest validation error: Line 1, Column 619, Reason: A <Resource> element is required for       packages targeting OS version 10.0.16299.0 or earlier.
            min_version: "10.0.16300.0".into(),
            max_version_tested: "10.0.20348.0".into(),
        }
    }
}

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-capabilities>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub enum Capability {
    /// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-capability>
    #[serde(rename(deserialize = "capability"))]
    Capability { name: String },
    /// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-rescap-capability>
    #[serde(rename(deserialize = "restricted"))]
    #[serde(rename(serialize = "rescap:Capability"))]
    Restricted { name: String },
    /// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-devicecapability>
    #[serde(rename(deserialize = "device"))]
    #[serde(rename(serialize = "DeviceCapability"))]
    Device { name: String },
    // TODO: uap:Capability and mobile:Capability
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(untagged, deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub enum ApplicationKind {
    #[default]
    Broken,
    Executable {
        executable: String,
        entry_point: String,
    },
    StartPage {
        ///  The web page that handles the extensibility point.
        start_page: String,
    },
}

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-application>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct Application {
    pub id: String,
    // #[serde(flatten)]
    // pub kind: ApplicationKind,
    pub executable: Option<String>,
    pub entry_point: Option<String>,
    #[serde(rename(serialize = "uap:VisualElements"))]
    pub visual_elements: VisualElements,
}

/// <https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap-visualelements>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all(serialize = "PascalCase"))]
pub struct VisualElements {
    pub display_name: String,
    pub description: String,
    pub background_color: String,
    #[serde(rename(serialize = "Square150x150Logo"))]
    pub logo_150x150: String,
    #[serde(rename(serialize = "Square44x44Logo"))]
    pub logo_44x44: String,

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
            display_name: "".into(),
            publisher_display_name: "".into(),
            logo: "".into(),
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
                display_name: "fluttertodoapp".into(),
                publisher_display_name: "com.flutter.fluttertodoapp".into(),
                logo: "Images\\StoreLogo.png".into(),
                description: Some("A new Flutter project.".into()),
            },
            resources: Resources {
                resource: vec![Resource {
                    language: Some("en".into()),
                    scale: None,
                    dx_feature_level: None,
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
                    id: "fluttertodoapp".into(),
                    executable: Some("todoapp.exe".into()),
                    entry_point: Some("Windows.FullTrustApplication".into()),
                    visual_elements: VisualElements {
                        background_color: "transparent".into(),
                        display_name: "fluttertodoapp".into(),
                        description: "A new flutter project.".into(),
                        logo_44x44: "Images\\Square44x44Logo.png".into(),
                        logo_150x150: "Images\\Square150x150Logo.png".into(),
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
