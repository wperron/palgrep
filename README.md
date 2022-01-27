# palgrep
A grep-style cli app that matches on palindromic lines

## Usage

To list all palindromes and their line number:

```bash
palgrep [FILES...]
```

To count the number of palindromes in each file:

```bash
palgrep [FILES...] | awk -F ':' '{print $1}' | uniq -c
```
