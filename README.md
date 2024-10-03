# Hayabusa encoded rules

This repository hosts two files that are going to be used with Hayabusa v2.18.0+ in order to minimize files that need to be put on a target system as well as bypass any false positives from anti-virus products. 
> At the time of writing, Windows Defender would alert with false-positives on a few Sigma `.yml` text files because they had malicious-looking keywords in them.
- `encoded_rules.yml`: This file stores all of the 4000+ Sigma and Hayabusa `.yml` rule files into a single file and XOR-encodes the data to prevent anti-virus products from falsely detecting on the rules.
- `rules_config_files.txt`: This file stores all of the rule config files (every `.yaml` config file in the `config` directory of the [hayabusa-rules](https://github.com/Yamato-Security/hayabusa-rules) repository) into a single file.

With Hayabusa v2.18.0+, after you place these two files in the root directory of the Hayabusa folder, you can safely delete the `rules` directory.
Hayabusa will load the rules and rule config files from these two files instead.
You can dynamically update the rules and config files with the `update-rules` command.

Both the rules and rule config files are updated on a daily basis.

The main purpose for this is when using Hayabusa for live-response where you do not want to cause any false positive alerting and you do not want to store many files on the target machine which may overwrite forensics artifacts like the USN journal.

When using Hayabusa on a DFIR analysis machine, you should use the regular package of Hayabusa that separates all of the rules and config files out.

# Deprecated encrypted package

We first were planning on using an encrypted zip file [rules.zip](https://github.com/Yamato-Security/hayabusa-encrypted-rules/raw/main/rules.zip) that contains the `config`, `hayabusa` and `sigma` directories of config files and detection rules hosted at the [hayabusa-rules](https://github.com/Yamato-Security/hayabusa-rules) repository, however, this would cause a performance degredation so we decided to opt for the XOR encoding instead.

* Password: `yamato-security-hayabusa`

You most likely do not need this file, but we are keeping it here anyway.
