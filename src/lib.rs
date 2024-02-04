use libc::{rlimit, setrlimit, RLIMIT_AS};
use std::{io::Result, os::unix::process::CommandExt, process::Command};

fn spawn_process(mem_limit: u64) -> Result<()> {
    let mut command = Command::new("./test");

    let limit = rlimit {
        rlim_cur: mem_limit,
        rlim_max: mem_limit,
    };
    unsafe {
        command.pre_exec(move || {
            setrlimit(RLIMIT_AS, &limit);
            Ok(())
        });
    }

    let _ = command.output()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::spawn_process;

    #[test]
    fn memory_exceed() {
        assert!(spawn_process(1).is_err());
    }

    #[test]
    fn should_pass() {
        assert!(spawn_process(100000).is_ok());
    }
}
