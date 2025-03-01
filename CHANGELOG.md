[2025/03/01] - Bug fix - The encoded rules file included rules that are supposed to be ignored by `ignore-uuid-list.txt`. (hayabusa#1596) (@fukusuket)

[2025/02/26] - `expand` rules are not being filtered out as they cannot be used for live response and require manual configuration beforehand. (hayabusa#1596) (@fukusuket)

[2025/02/14] - Bug fix - The `RulePath` was blank in the encoded rules when there were multiple rules in a single file. (hayabusa#1572) (@fukusuket)

[2025/02/10] - Bug fix - Number of rules enabled after channel filtering differed between live response and standard hayabusa. (hayabusa#1557) (@fukusuket)

[2025/02/01] - Bug fix - Rule file names were off by one. (hayabusa#1555) (@fukusuket)

[2024/10/29] - Fixed a panic bug due to multiple YAML files being contained in a single file. (#4) (@fukusuket)

[2024/10/03] - Rules config files are now combined into a single file.
