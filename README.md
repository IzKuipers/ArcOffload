# ArcOffload
The official utility to download all your ArcFS files from any ArcAPI.

Downloads can be found on the [releases](https://github.com/IzKuipers/ArcOffload/releases/latest) page.

## Usage
Windows, Linux and MacOS are all supported. [Download](https://github.com/IzKuipers/ArcOffload/releases/latest) the executable, and run `./arc-offload` (`.\arc-offload.exe` for Windows) from the terminal. Type `./arc-offload -h` to display the flags you can specify:

```
$ ./arc-offload -h
Usage: arc-offload [OPTIONS] --server <SERVER>

Options:
  -s, --server <SERVER>            The ArcAPI to connect to
  -a, --authcode <AUTHCODE>        The Authcode, if any (used for protected servers) [default: ]
  -p, --port <PORT>                The TCP port of the server [default: 3333]
  -i, --is-https                   Specify if you want to connect using HTTPS
  -d, --destination <DESTINATION>  The directory to download the files to [default: out]
  -c, --cred_from_env              Read the credentials from a `.env` file
  -h, --help                       Print help
  -V, --version                    Print version
```

For example, to download files from the Community API, use `./arc-offload -s community.arcapi.nl -i`. Once connected you'll be asked for a username and password. Here you'll enter your ArcOS credentials. The utility will then connect with the server to authenticate you, after which it'll ask if you want to proceed. Enter `y` (or `Y`) to do confirm.

## The `.env` file
Starting with version 1.0.1, it's possible to read the ArcOS account credentials from a `.env` file, instead of being prompted for them. To do this, be sure to specify the `-c` flag, and create a `.env` file as such:
```
OFFUSR="username"
OFFPWD="password"
```
The double-quotes are optional unless you have a space in either the username or the password. Be sure to never upload this file anywhere, as it contains sensitive information. When specifying the `-c` flag, ArcOffload grabs the credentials from `.env` and skips the download confirmation. You can use this in conjunction with a cronjob to periodically offload your ArcOS files, for example.

## Example
An example usage might be:

```
$ ./arc-offload -s community.arcapi.nl -i
    _           ___   __  __ _              _ 
   /_\  _ _ __ / _ \ / _|/ _| |___  __ _ __| |
  / _ \| '_/ _| (_) |  _|  _| / _ \/ _` / _` |
 /_/ \_\_| \__|\___/|_| |_| |_\___/\__,_\__,_|

ArcOffload 1.0.0. Created by Izaak Kuipers for the ArcOS Project.

Calling up community.arcapi.nl on port 443 (presumably Public)...

Username: Izaak Kuipers
Password: 

I will now start writing 6 file(s) to out. Confirm? [y/n]: y

[1/6] Writing: LogDump-892357138.txt
[2/6] Writing: arcterm.conf
[3/6] Writing: LogDump-827532080.txt
[4/6] Writing: 2048-1.png
[5/6] Writing: Themes/cube.arctheme
[6/6] Writing: Messages/Message from Izaak Kuipers - 680199686.md

```

## Notices
- An artficial delay is introduced between file downloads to prevent a rate limit from occuring. Because of this, large ArcFS downloads may take a really long time.
- The utility **does not** have support for downloading a specific folder instead of everything. It's all or nothing.
- Please report any bugs you find on the [issues](https://github.com/IzKuipers/ArcOffload/issues) page so that I can fix them in a new release.

  This utility is written by the creator of ArcOS, but the ArcOS team will not give support for it in the ArcOS Community Discord Server. The Issues page is the only place where I'll give support.

## Author
[Izaak Kuipers](https://github.com/IzKuipers) <izaak.kuipers@gmail.com>

## License
MIT
