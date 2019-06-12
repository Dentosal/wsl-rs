# Detect if the code is ran under Windows Subsystem for Linux

## Usage

`wsl::is_wsl()` returns true under WSL, false otherwise.

## Method

[A semi-official source](https://github.com/microsoft/WSL/issues/423#issuecomment-221627364) lists some pointers. We are going to use the simplest approach here: Testing if `/proc/sys/kernel/osrelease` contains string `Microsoft` or `WSL`.
