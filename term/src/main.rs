use nix::pty::forkpty;
use nix::unistd::ForkResult;
use nix::unistd::read;
use std::os::unix::io::RawFd;
use std::process::Command;


fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65535];
    let read_result = read(fd, &mut read_buffer);
    
    match read_result {
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_e) => None,
    }
}

fn spawn_pty_with_shell(default_shell: String) -> RawFd {
    unsafe {
        // forkpty is a libc function that will fork our current process. This process being our
        // pty and sending it back to the secondary side of the pty 
        // apperently we use nix to handle the "unsafe" part of the code but it made we wrap it in
        // unsafe block
        match forkpty(None, None) {
            Ok(fork_pty_res) => {
                // this code will execute/run in both the primary pty and secondary
                let stdout_fd = fork_pty_res.master; // primary shell
                if let ForkResult::Child = fork_pty_res.fork_result {
                    Command::new(&default_shell)
                        .spawn()
                        .expect("[ ERROR ] Failed to spawn shell.");
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    std::process::exit(0);
                }
                stdout_fd
            }
            Err(e) => {
                panic!("[ ERROR ] Failed to fork {:?}", e);
            }
        }
    }
}


fn main() {
    let default_shell = std::env::var("SHELL")
        .expect("[ ERROR ] Coult not find default shell from $SHELL");

    let stdout_fd = spawn_pty_with_shell(default_shell);
    let mut read_buffer = vec![];

    loop {
        match read_from_fd(stdout_fd) {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            }
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(0);
            }
        }
    }
    
}
