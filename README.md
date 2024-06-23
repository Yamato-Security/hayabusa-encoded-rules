# Hayabusa Encrypted Rules

This repository hosts an encrypted [rules.zip](https://github.com/Yamato-Security/hayabusa-encrypted-rules/raw/main/rules.zip) zip file that contains the `config`, `hayabusa` and `sigma` directories of config files and detection rules hosted at the [hayabusa-rules](https://github.com/Yamato-Security/hayabusa-rules) repository.

* Password: `yamato-security-hayabusa`

Windows Defender and probably other anti-virus software will sometimes give false positives on sigma rules because they contain keywords such as `mimikatz` inside the `.yml` files.
In order to run Hayabusa on endpoints and avoid false positives we host the encrypted `rules.zip` file so that Hayabusa will download and use encrypted rules.
This is mainly to be used for with the [Velociraptor artifact](https://docs.velociraptor.app/exchange/artifacts/pages/windows.eventlogs.hayabusa/) but can and should be used anytime you run Hayabusa for live response and cannot or do not want to disable the anti-virus, etc...
By gathering the 4000+ rules together in one file, this also minimizes impact on forensics artifacts such as the USN journal.
