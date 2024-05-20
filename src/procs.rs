use sysinfo::{Pid, ProcessRefreshKind, System};

pub fn get_procs(sys: &mut System) -> Vec<(Pid, String)> {
    sys.refresh_processes_specifics(
        ProcessRefreshKind::new().with_cmd(sysinfo::UpdateKind::Always),
    );
    let mut v = sys
        .processes()
        .into_iter()
        .map(|(&pid, proc)| {
            (
                pid,
                proc.cmd()
                    .into_iter()
                    .flat_map(|s| [String::as_str(s), " "])
                    .collect::<String>(),
            )
        })
        .collect::<Vec<_>>();
    v.sort_by(|x, y| x.1.cmp(&y.1));
    v
}
