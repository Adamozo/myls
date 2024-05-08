# myls

This program works like classical unix `ls` with filters, icons, directories tree and sorting.
To use program you can just type `./myls` and It will show you available options.
Program was builded on linux and may not work on windows.
Detailed os version:

```
Distributor ID: Linuxmint
Description:    Linux Mint 21.3
Release:        21.3
Codename:       virginia
```

To run and test program on other linux distributions just use cargo and it should work well.

## Available options

```
Usage: myls [OPTIONS] [PATH]

Arguments:
  [PATH]  [default: .]

Options:
      --min <BYTES>    Filters files that are at least a certain size in bytes
      --max <BYTES>    Filters files that are at most a certain size in bytes
  -r, --reg <REG>      Filters files that are at most a certain size in bytes
  -n, --no-hidden      Excludes hidden files from the results
  -l, --long           Display results in long format
  -f, --files          Display only files
  -d, --directories    Display only directories
      --depth <BYTES>  Display tree structure with given depth
  -s, --sort <SORT>    Select sorting method [default: Name]
  -h, --help           Print help
  -V, --version        Print version
```

### Filters

- Filters files that are at least a certain size in bytes
- Filters files that are at most a certain size in bytes
- Filters entries that match with given regex
- Excludes hidden files from the results

```
./myls --min 2000
./myls --max 20
./myls -r "\.toml$"
./myls -n -l
```

### Sorters

- By date
- By name
- By size

```
./myls -s name
./myls -l -s date
./myls -l -s size
```

### Display options

- Display results in long or short format. Short by default
  `./myls -l test_directory`
- Display only files `./myls -f test_directory`
- Display only directories `./myls -d test_directory`
- Display tree structure with given depth `./myls --depth 2 test_directory`
