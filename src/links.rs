use std::collections::HashSet;

use proc_maps::{get_process_maps, Pid};

pub fn get_links(pid: Pid) -> anyhow::Result<HashSet<String>> {
    let res = get_process_maps(pid)?;
    Ok(res
        .into_iter()
        .filter_map(|m| {
            if m.is_read() {
                m.filename().map(|p| p.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect())
}
