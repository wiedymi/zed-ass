use zed_extension_api::{self as zed, Result};

struct AssExtension {
    cached_binary_path: Option<String>,
}

impl AssExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if std::fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "wiedymi/ass-lsp",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: true,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = match (platform, arch) {
            (zed::Os::Mac, zed::Architecture::Aarch64) => "ass-lsp-aarch64-apple-darwin.tar.gz",
            (zed::Os::Mac, zed::Architecture::X8664) => "ass-lsp-x86_64-apple-darwin.tar.gz",
            (zed::Os::Linux, zed::Architecture::X8664) => "ass-lsp-x86_64-unknown-linux-gnu.tar.gz",
            (zed::Os::Windows, zed::Architecture::X8664) => "ass-lsp-x86_64-pc-windows-msvc.zip",
            _ => return Err(format!("Unsupported platform: {:?} {:?}", platform, arch)),
        };

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("ass-lsp-{}", release.version);
        let binary_path = format!(
            "{version_dir}/{}",
            if matches!(platform, zed::Os::Windows) {
                "ass-lsp.exe"
            } else {
                "ass-lsp"
            }
        );

        if !std::fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let download_file_type = if asset_name.ends_with(".zip") {
                zed::DownloadedFileType::Zip
            } else {
                zed::DownloadedFileType::GzipTar
            };

            zed::download_file(&asset.download_url, &version_dir, download_file_type)
                .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries = std::fs::read_dir(".")
                .map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    std::fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for AssExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, _worktree)?,
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(AssExtension);
