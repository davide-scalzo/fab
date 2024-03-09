# WIP

Small cli utility that enables managing multiple instances of a binary. Lots to do still, use at your own risk.

Two sample binaries are provided, both compiled for Intel MacOs, if on a different platform compile first he project with `cargo build --release` and use that executable instead.
For convenience you can add the binary to your path.

## Usage
`fab up ./test-server` starts an instance of test-server. The test-server binary is configure to listen on port 3000 or pick the next available if trying to open a non-available port.
```
💅 test-server started successfully. PID: 60669
```

`fab down test-server` stops a test-server instance

`fab status` shows currently running instances managed by `fab`
```
Name            PID     Ports   CPU     Memory
test-server     60669   *:3000  0       1
```
