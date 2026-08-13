use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// Compute SHA-256 of arbitrary bytes, return lowercase hex.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

/// Compute SHA-256 of a UTF-8 string (e.g. for canonical JSON).
pub fn sha256_str(s: &str) -> String {
    sha256_hex(s.as_bytes())
}

/// Read the current executable's path and compute its SHA-256.
/// Returns None if the binary cannot be found or read.
pub fn binary_digest() -> Option<String> {
    std::env::current_exe()
        .ok()
        .and_then(|p| std::fs::read(p).ok())
        .map(|b| sha256_hex(&b))
}

/// Read peak RSS from /proc/self/status on Linux, sysctl/getrusage on macOS.
/// Returns 0 if unavailable.
pub fn peak_rss_kb() -> usize {
    #[cfg(target_os = "linux")]
    {
        if let Ok(status) = std::fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if line.starts_with("VmHWM:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        if let Ok(kb) = parts[1].parse::<usize>() {
                            return kb;
                        }
                    }
                }
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        #[repr(C)]
        struct timeval {
            tv_sec: i64,
            tv_usec: i32,
            _pad: i32,
        }
        #[repr(C)]
        struct rusage {
            ru_utime: timeval,
            ru_stime: timeval,
            ru_maxrss: i64, // on macOS, this is in bytes
            ru_ixrss: i64,
            ru_idrss: i64,
            ru_isrss: i64,
            ru_minflt: i64,
            ru_majflt: i64,
            ru_nswap: i64,
            ru_inblock: i64,
            ru_oublock: i64,
            ru_msgsnd: i64,
            ru_msgrcv: i64,
            ru_nsignals: i64,
            ru_nvcsw: i64,
            ru_nivcsw: i64,
        }
        extern "C" {
            fn getrusage(who: std::os::raw::c_int, usage: *mut rusage) -> std::os::raw::c_int;
        }

        let mut usage = std::mem::MaybeUninit::<rusage>::uninit();
        unsafe {
            if getrusage(0, usage.as_mut_ptr()) == 0 {
                let usage = usage.assume_init();
                return (usage.ru_maxrss / 1024) as usize;
            }
        }
    }
    0
}

/// Get git HEAD SHA of the repo at `repo_path`.
/// Reads .git/HEAD and resolves refs/heads/... to the SHA.
/// Returns "unknown" if the repo cannot be read.
pub fn repo_commit(repo_path: &str) -> String {
    let repo_path = PathBuf::from(repo_path);
    let head_path = repo_path.join(".git").join("HEAD");

    let head_content = match std::fs::read_to_string(&head_path) {
        Ok(content) => content.trim().to_string(),
        Err(_) => return "unknown".to_string(),
    };

    if head_content.starts_with("ref:") {
        let ref_subpath = head_content.strip_prefix("ref:").unwrap().trim();
        let ref_path = repo_path.join(".git").join(ref_subpath);
        if let Ok(sha) = std::fs::read_to_string(&ref_path) {
            let sha = sha.trim();
            if sha.len() == 40 {
                return sha.to_string();
            }
        } else {
            // Check packed-refs
            let packed_path = repo_path.join(".git").join("packed-refs");
            if let Ok(packed_content) = std::fs::read_to_string(&packed_path) {
                for line in packed_content.lines() {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() == 2 && parts[1] == ref_subpath && parts[0].len() == 40 {
                        return parts[0].to_string();
                    }
                }
            }
        }
    } else if head_content.len() == 40 {
        return head_content;
    }

    "unknown".to_string()
}
