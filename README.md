<div align="center">

# minicat
A small cat clone, with pretty printing, inspired by [bat](https://github.com/sharkdp/bat)

![](https://img.shields.io/github/last-commit/loenard97/minicat?&style=for-the-badge&color=F74C00)
![](https://img.shields.io/github/repo-size/loenard97/minicat?&style=for-the-badge&color=F74C00)

</div>


## Usage
Output contents of a file to stdout:
```txt
$ minicat .\poem.txt
─────┬─────────────────────────────────────────────────────
     │  File .\poem.txt
─────┼─────────────────────────────────────────────────────
  1  │  I'm nobody! Who are you?
  2  │  Are you nobody, too?
  3  │  Then there's a pair of us - don't tell!
  4  │  They'd banish us, you know.
  5  │
  6  │  How dreary to be somebody!
  7  │  How public, like a frog
  8  │  To tell your name the livelong dreary
  9  │  To an admiring bog!
─────┴─────────────────────────────────────────────────────
```

Or use stdin as input:
```sh
$ echo "Hello World!" | minicat
─────┬─────────────────────────────────────────────────────
     │  File <stdin>
─────┼─────────────────────────────────────────────────────
  1  │  "Hello World!"
─────┴─────────────────────────────────────────────────────
```

Pretty printing is automatically disabled when redirecting stdout.
