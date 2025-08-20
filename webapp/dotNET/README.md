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

## debugging URL

http://127.0.0.1:5123/swagger/
