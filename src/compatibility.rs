//! Upstream compatibility policy, not a runtime proof of the Airlift primitive.
//! Sources: README.md and ios-bundles/CarrierSIM (27.2 beta build identifiers).

pub const PROBE_LIMITATION: &str = "ReadyForSync verifies the handshake only; it does not verify Airlift file writes or iOS compatibility.";

pub fn write_block_reason(version: &str, build: &str) -> Option<String> {
    let parsed = parse_version(version);
    if let Some((major, minor, patch)) = parsed {
        if (18..=26).contains(&major)
            || (major == 27 && minor == 0 && patch <= 1)
            || ((major, minor, patch) == (27, 2, 0) && matches!(build, "24B5084k" | "24B5089g"))
        {
            return None;
        }
        if (major, minor, patch) >= (27, 2, 0) {
            return Some(format!(
                "iOS {version} [{build}] is outside the supported Airlift range. \
                 Upstream reports that Apple patched this method starting with iOS 27.2 beta 3; \
                 27.2 RC is not supported. Protected file operations are blocked before AFC staging or cleanup. \
                 {PROBE_LIMITATION}"
            ));
        }
    }
    Some(format!(
        "Cannot verify Airlift compatibility for iOS {version} [{build}]. \
         Supported range: iOS 18.0-27.0.1 and the known 27.2 beta 1-2 builds only. \
         Protected file operations are blocked before AFC staging or cleanup."
    ))
}

fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let mut parts = version.split('.');
    let mut number = || {
        let s = parts.next()?;
        (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())).then(|| s.parse::<u32>().ok())?
    };
    let major = number()?;
    let minor = number()?;
    // ProductVersion normally has two or three components.
    let patch = match parts.next() {
        None => 0,
        Some(s) if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) => s.parse().ok()?,
        _ => return None,
    };
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

#[cfg(test)]
mod tests {
    use super::write_block_reason;

    #[test]
    fn reported_release_candidate_is_blocked() {
        let reason = write_block_reason("27.2", "24B115").expect("RC must stop before staging");
        assert!(reason.contains("27.2 [24B115]"));
        assert!(reason.contains("beta 3"));
    }

    #[test]
    fn known_early_betas_are_not_confused_with_rc() {
        for build in ["24B5084k", "24B5089g"] {
            assert!(write_block_reason("27.2", build).is_none(), "{build}");
        }
        for build in ["24B5099f", "24B5110a", "24B115", "Unknown", ""] {
            assert!(write_block_reason("27.2", build).is_some(), "{build}");
        }
    }

    #[test]
    fn supported_range_remains_available() {
        for version in ["18.0", "18.7.1", "26.6", "27.0", "27.0.1"] {
            assert!(
                write_block_reason(version, "Unknown").is_none(),
                "{version}"
            );
        }
    }

    #[test]
    fn unknown_versions_do_not_enable_protected_file_operations() {
        for version in [
            "", "Unknown", "27.x", "27.2 RC", "17.9", "27.0.2", "27.1", "27.2.1", "27.3", "28.0",
        ] {
            assert!(
                write_block_reason(version, "24B5084k").is_some(),
                "{version}"
            );
        }
    }
}
