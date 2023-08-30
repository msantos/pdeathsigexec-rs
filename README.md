# SYNOPSIS

pdeathsigexec *cmd* *...*

# DESCRIPTION

pdeathsigexec: signal process when parent exits

A subprocess whose parent exits may be re-parented to init (PID 1)
and continue to run. `pdeathsigexec` sets the process to have a signal
sent if the parent process terminates.

The "signal on parent termination" behaviour applies
to the executed process only and not descendents
([prctl(2)](https://man7.org/linux/man-pages/man2/prctl.2.html)):

```
The parent-death signal setting is cleared for the child of a fork(2).
It is also (since Linux 2.4.36 / 2.6.23) cleared when  executing  a
set-user-ID or set-group-ID binary, or a binary that has associated
capabilities (see capabilities(7)); otherwise, this value is preserved
across execve(2).
```

# EXAMPLES

```
$ sh -c "sleep inf" &
[1] 25880
$ kill -9 25880
$ pgrep -fa sleep
25882 sleep inf
```

## pdeathsigexec

```
$ sh -c "pdeathsigexec sleep inf" &
[1] 25926
$ kill -9 25926
$ pgrep -fa sleep
<no output>
```

# Build

```
cargo build
```

# OPTIONS

## pdeathsigexec

-s/--signal
: set the termination signal (default `9` (SIGKILL))

# ENVIRONMENT VARIABLES

None.
