# Purpose

A viewer for displaying XML files in a collapsible format. This is designed to only work with Entrez XML files.

This program was designed to solve my issue of not having a unified interface to search and browse biological
data or annotations. This program is meant to be a companion to [ncbi-rs](https://github.com/PoorRican/ncbi-rs]
and is able to view the XML files that are downloaded from the Entrez database.

## Usage

Without any command line args, the program will present prompt a file path input. The input will accept relative
and absolute paths. The program will then parse the file and display the data in a collapsible format. There 
is the ability to specify a file path as a command line argument.


Here is the help page
```plaintext
Browse biological annotation data

Usage: browsr [COMMAND]

Commands:
  view  Directly view annotation data
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```


To view a specific file, you can use the `view` command. Here is the help page for the `view` command
```bash
browsr view 2519734247.xml
```

### Exiting

In order to exit the program, you can press `q` to exit the program.