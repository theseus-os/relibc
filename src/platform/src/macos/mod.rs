use types::*;
use *;

const MAXPATHLEN: usize = 1024;

pub fn e(sys: usize) -> usize {
    if (sys as isize) < 0 && (sys as isize) >= -256 {
        unsafe {
            errno = -(sys as isize) as c_int;
        }
        !0
    } else {
        sys
    }
}

pub unsafe fn accept(socket: c_int, address: *mut sockaddr, address_len: *mut socklen_t) -> c_int {
    e(syscall!(ACCEPT, socket, address, address_len)) as c_int
}
pub unsafe fn bind(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int {
    e(syscall!(BIND, socket, address, address_len)) as c_int
}
pub fn brk(addr: *mut c_void) -> *mut c_void {
    // mac has no good solution for this
    e(-38isize as usize) as *mut c_void
}
pub fn chdir(path: *const c_char) -> c_int {
    e(unsafe { syscall!(CHDIR, path) }) as c_int
}
pub fn chmod(path: *const c_char, mode: mode_t) -> c_int {
    e(unsafe { syscall!(CHMOD, path, mode) }) as c_int
}
pub fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int {
    e(unsafe { syscall!(CHOWN, path, owner as u32, group as u32) }) as c_int
}
pub fn clock_gettime(clk_id: clockid_t, tp: *mut timespec) -> c_int {
    // FIXME implement this, no single mac syscall
    e(-38isize as usize) as c_int
}
pub fn close(fildes: c_int) -> c_int {
    e(unsafe { syscall!(CLOSE, fildes) }) as c_int
}
pub unsafe fn connect(socket: c_int, address: *const sockaddr, address_len: socklen_t) -> c_int {
    e(syscall!(CONNECT, socket, address, address_len)) as c_int
}
pub fn dup(fildes: c_int) -> c_int {
    e(unsafe { syscall!(DUP, fildes) }) as c_int
}
pub fn dup2(fildes: c_int, fildes2: c_int) -> c_int {
    e(unsafe { syscall!(DUP2, fildes, fildes2) }) as c_int
}
pub fn execve(path: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int {
    e(unsafe { syscall!(EXECVE, path, argv, envp) }) as c_int
}
pub fn exit(status: c_int) -> ! {
    unsafe {
        syscall!(EXIT, status);
    }
    loop {}
}
pub fn fchdir(fildes: c_int) -> c_int {
    e(unsafe { syscall!(FCHDIR, fildes) }) as c_int
}
pub fn fchmod(fildes: c_int, mode: mode_t) -> c_int {
    e(unsafe { syscall!(FCHMOD, fildes, mode) }) as c_int
}
pub fn fchown(fildes: c_int, owner: uid_t, group: gid_t) -> c_int {
    e(unsafe { syscall!(FCHOWN, fildes, owner, group) }) as c_int
}
pub fn fcntl(fildes: c_int, cmd: c_int, arg: c_int) -> c_int {
    e(unsafe { syscall!(FCNTL, fildes, cmd, arg) }) as c_int
}
pub fn fcntl_ptr(fildes: c_int, cmd: c_int, arg: *mut c_void) -> c_int {
    e(unsafe { syscall!(FCNTL, fildes, cmd, arg) }) as c_int
}
pub fn fork() -> usize {
    let pid: usize;
    let is_child: usize;
    unsafe {
        asm!("syscall" : "={eax}"(pid), "={edx}"(is_child)
                       : "{eax}"(sc::platform::syscall_construct_unix(sc::platform::nr::FORK))
                       : "rcx", "r11", "memory"
                       : "volatile");
    }
    // AFTER FORK SYSCALL
    // if there was a problem, CF==1 and errno is in %rax
    // else, if parent, %edx is 0 and %eax contains the child pid
    // else (child), %edx is nonzero
    // macos libsyscall adjusts the stack pointer for some reason
    if (pid as isize) < 0 { // something went wrong
        e(pid)
    } else if is_child == 0 { // in parent
        pid
    } else { // in child
        // TODO clear cached getpid
        0
    }
}
pub fn fstat(fildes: c_int, buf: *mut stat) -> c_int {
    e(unsafe { syscall!(FSTAT, fildes, buf) }) as c_int
}
pub fn fsync(fildes: c_int) -> c_int {
    e(unsafe { syscall!(FSYNC, fildes) }) as c_int
}
pub fn ftruncate(fildes: c_int, length: off_t) -> c_int {
    e(unsafe { syscall!(FTRUNCATE, fildes, length) }) as c_int
}
pub fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char {
    const F_GETPATH: c_int = 50;
    let mut dotpath = [0u8; 1024];
    let dot = open(b".\0".as_ptr() as *const i8, 0, 0);
    let res = fcntl_ptr(dot, F_GETPATH, dotpath.as_mut_ptr() as *mut c_void);
    if res == -1 {
        ptr::null_mut()
    } else if let Some(index) =
        dotpath
            .iter()
            .position(|x| *x == 0)
            .and_then(|x| if x < size { Some(x) } else { None })
    {
        let mut out = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, index + 1) };
        out.copy_from_slice(&dotpath[..index + 1]);
        buf
    } else {
        unsafe { errno = 34 };
        ptr::null_mut()
    }
}
pub fn getegid() -> gid_t {
    e(unsafe { syscall!(GETEGID) })
}
pub fn geteuid() -> uid_t {
    e(unsafe { syscall!(GETEUID) })
}
pub fn getgid() -> gid_t {
    e(unsafe { syscall!(GETGID) })
}
pub unsafe fn getpeername(
    socket: c_int,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> c_int {
    e(syscall!(GETPEERNAME, socket, address, address_len)) as c_int
}
pub fn getpgid(pid: pid_t) -> pid_t {
    e(unsafe { syscall!(GETPGID, pid) })
}
pub fn getpid() -> pid_t {
    e(unsafe { syscall!(GETPID) })
}
pub fn getppid() -> pid_t {
    e(unsafe { syscall!(GETPPID) })
}
pub unsafe fn getsockname(
    socket: c_int,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> c_int {
    e(syscall!(GETSOCKNAME, socket, address, address_len)) as c_int
}
pub fn getsockopt(
    socket: c_int,
    level: c_int,
    option_name: c_int,
    option_value: *mut c_void,
    option_len: *mut socklen_t,
) -> c_int {
    e(unsafe {
        syscall!(
            GETSOCKOPT,
            socket,
            level,
            option_name,
            option_value,
            option_len
        )
    }) as c_int
}
pub fn getuid() -> uid_t {
    e(unsafe { syscall!(GETUID) })
}
pub fn kill(pid: pid_t, sig: c_int) -> c_int {
    e(unsafe { syscall!(KILL, pid, sig) }) as c_int
}
pub fn killpg(pgrp: pid_t, sig: c_int) -> c_int {
    e(unsafe { syscall!(KILL, -(pgrp as isize) as pid_t, sig) }) as c_int
}
pub fn link(path1: *const c_char, path2: *const c_char) -> c_int {
    e(unsafe { syscall!(LINK, path1, path2) }) as c_int
}
pub fn listen(socket: c_int, backlog: c_int) -> c_int {
    e(unsafe { syscall!(LISTEN, socket, backlog) }) as c_int
}
pub fn lseek(fildes: c_int, offset: off_t, whence: c_int) -> off_t {
    e(unsafe { syscall!(LSEEK, fildes, offset, whence) }) as off_t
}
pub fn lstat(file: *const c_char, buf: *mut stat) -> c_int {
    e(unsafe { syscall!(LSTAT, file, buf) }) as c_int
}
pub fn mkdir(path: *const c_char, mode: mode_t) -> c_int {
    e(unsafe { syscall!(MKDIR, path, mode) }) as c_int
}
pub fn mkfifo(path: *const c_char, mode: mode_t) -> c_int {
    e(unsafe { syscall!(MKFIFO, path, mode) }) as c_int
}
pub fn mmap(addr: *mut c_void, len: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void {
    e(unsafe { syscall!(MMAP, addr, len, prot, flags, fd, offset) }) as *mut c_void
}
pub fn munmap(addr: *mut c_void, len: size_t) -> c_int {
    e(unsafe { syscall!(MUNMAP, addr, len) }) as c_int
}
pub fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> c_int {
    // FIXME implement this
    e(-38isize as usize) as c_int
}
pub fn open(path: *const c_char, oflag: c_int, mode: mode_t) -> c_int {
    e(unsafe { syscall!(OPEN, path, oflag, mode) }) as c_int
}
pub fn pipe(fildes: &mut [c_int]) -> c_int {
    // PIPE is "special", it places a second return value in %edx
    // thus we need custom asm
    let ret: usize;
    unsafe {
        asm!("syscall" : "={eax}"(fildes[0]), "={edx}"(fildes[1]), "={rax}"(ret)
                       : "{rax}"(sc::platform::syscall_construct_unix(sc::platform::nr::PIPE))
                       : "rcx", "r11", "memory"
                       : "volatile");
    }
    let err = e(ret);
    if err == !0 {
        err as c_int
    } else {
        0
    }
}
pub fn read(fildes: c_int, buf: &mut [u8]) -> ssize_t {
    e(unsafe { syscall!(READ, fildes, buf.as_mut_ptr(), buf.len()) }) as ssize_t
}
pub unsafe fn recvfrom(
    socket: c_int,
    buf: *mut c_void,
    len: size_t,
    flags: c_int,
    address: *mut sockaddr,
    address_len: *mut socklen_t,
) -> ssize_t {
    e(syscall!(
        RECVFROM,
        socket,
        buf,
        len,
        flags,
        address,
        address_len
    )) as ssize_t
}
pub fn rename(old: *const c_char, new: *const c_char) -> c_int {
    e(unsafe { syscall!(RENAME, old, new) }) as c_int
}
pub fn rmdir(path: *const c_char) -> c_int {
    e(unsafe { syscall!(RMDIR, path) }) as c_int
}
pub unsafe fn sendto(
    socket: c_int,
    buf: *const c_void,
    len: size_t,
    flags: c_int,
    dest_addr: *const sockaddr,
    dest_len: socklen_t,
) -> ssize_t {
    e(syscall!(
        SENDTO, socket, buf, len, flags, dest_addr, dest_len
    )) as ssize_t
}
pub fn setpgid(pid: pid_t, pgid: pid_t) -> c_int {
    e(unsafe { syscall!(SETPGID, pid, pgid) }) as c_int
}
pub fn setregid(rgid: gid_t, egid: gid_t) -> c_int {
    e(unsafe { syscall!(SETREGID, rgid, egid) }) as c_int
}
pub fn setreuid(ruid: uid_t, euid: uid_t) -> c_int {
    e(unsafe { syscall!(SETREUID, ruid, euid) }) as c_int
}
pub fn setsockopt(
    socket: c_int,
    level: c_int,
    option_name: c_int,
    option_value: *const c_void,
    option_len: socklen_t,
) -> c_int {
    e(unsafe {
        syscall!(
            SETSOCKOPT,
            socket,
            level,
            option_name,
            option_value,
            option_len
        )
    }) as c_int
}
pub fn shutdown(socket: c_int, how: c_int) -> c_int {
    e(unsafe { syscall!(SHUTDOWN, socket, how) }) as c_int
}
pub fn stat(file: *const c_char, buf: *mut stat) -> c_int {
    e(unsafe { syscall!(STAT, file, buf) }) as c_int
}
pub fn socket(domain: c_int, kind: c_int, protocol: c_int) -> c_int {
    e(unsafe { syscall!(SOCKET, domain, kind, protocol) }) as c_int
}
pub fn socketpair(domain: c_int, kind: c_int, protocol: c_int, socket_vector: *mut c_int) -> c_int {
    e(unsafe { syscall!(SOCKETPAIR, domain, kind, protocol, socket_vector) }) as c_int
}
pub fn uname(utsname: usize) -> c_int {
    // FIXME implement this
    e(-38isize as usize) as c_int
}
pub fn unlink(path: *const c_char) -> c_int {
    e(unsafe { syscall!(UNLINK, path) }) as c_int
}
pub fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t {
    e(unsafe { syscall!(WAIT4, pid, stat_loc, options, 0) }) as pid_t
}
pub fn write(fildes: c_int, buf: &[u8]) -> ssize_t {
    e(unsafe { syscall!(WRITE, fildes, buf.as_ptr(), buf.len()) }) as ssize_t
}
