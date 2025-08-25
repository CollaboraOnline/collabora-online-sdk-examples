Please see [../README.md](../README.md) for usage and general information.

## Requirements

[dotNET (.NET)](https://dotnet.microsoft.com/) version 9.0.

Installation:
* Fedora-Linux-42: `dnf install dotnet-sdk-9.0`
* For most other operating systems use: https://learn.microsoft.com/dotnet/core/install

## Run the example

`dotnet run [OPTION]...`

Options get passed as "args" in "Program.cs".

* listen address: `--urls http://0.0.0.0:8080`
* mode: `--environment { Development | Production }`
  * Setting the mode requires removing that setting from `Properties/launchSettings.json`

This example offers WOPI-Proof validation and outputs the results to STDOUT.  
To use WOPI-Proof when running behind a reverse HTTP proxy, set the following environment variable like this.  
`REVERSE_PROXY_URL_PREFIX="https://public-name.example.org/subdir"`  
Do NOT append a trailing slash!

## debugging URL

http://127.0.0.1:5123/swagger/
