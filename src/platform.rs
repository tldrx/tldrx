#[derive(Debug, PartialEq, Clone)]
pub enum Platform {
    Common,
    Linux,
    Osx,
    Windows,
    Android,
    Sunos,
    Other(String),
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Platform::Common => "common",
            Platform::Linux => "linux",
            Platform::Osx => "osx",
            Platform::Windows => "windows",
            Platform::Android => "android",
            Platform::Sunos => "sunos",
            Platform::Other(p) => p,
        };
        write!(f, "{}", text)
    }
}

impl std::str::FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let p = match s {
            "linux" => Platform::Linux,
            "osx" | "macos" => Platform::Osx,
            "windows" => Platform::Windows,
            "android" => Platform::Android,
            "sunos" => Platform::Sunos,
            "" => Default::default(),
            _ => Platform::Other(s.to_string()),
        };
        Ok(p)
    }
}

impl Default for Platform {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(any(
            target_os = "macos",
            target_os = "ios"
        ))]
        return Platform::Osx;

        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(target_os = "android")]
        return Platform::Android;

        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "ios",
            target_os = "windows",
            target_os = "android"
        )))]
        return Platform::Other("common");
    }
}
