# Purple

Microkernel

# Overall Structure

* purple
    * kernel
        * physical memory access abstraction
        * process management
            * scheduling
            * capabilities (selinux)
            * trusted (physical memory access abstraction)
        * ipc (interprocess communication)
            * messages
                * register abstraction
                    * fs register - allow servers to register paths (`/dev/serial1`)
                    * network register - allow servers to register to calls like `socket(AF_INET)`
    * usermode processes
        * libc emulator
        * trusted - access physical memory
            * filesystems
            * network stacks
            * display
        * untrusted
            * sql server
            * browser
            * etc

This projects aims at being a microkernel and simple.

Having unix traits of _everything is file philosophy_ to the extremes of plan9
without a per process namespace.

It also aims at being useful thus making the ease of programming worth
against the lower performance produced by not being monolithic.

## Posix Compliance

Usefulness is being posix compliance. It allows using existing programs and allows
unix programmers to easily develop programs.
It can be achieved by an emulator which is practically a libc implementation that writes and reads from files
instead of sending syscalls.


```
┌────────────────────────────────────────────────────────┬───────────────────────────────────────────┐
│libc call                                               │implementation                             │
├────────────────────────────────────────────────────────┼───────────────────────────────────────────┤
│int connect(my_sock_fd, "192.168.1.12:8000", IPV4_SIZE) │sock_fd = open("/net/tcp/192.168.1.12/8000)│
├────────────────────────────────────────────────────────┼───────────────────────────────────────────┤
│int send(my_sock_fd, "hello world", 10);                │write(sock_fd, "hello world", 10);         │
└────────────────────────────────────────────────────────┴───────────────────────────────────────────┘
```

Because the kernel is a microkernel the calls to `open` and `write` don't initiate syscalls but rather
send messages as IPC.
