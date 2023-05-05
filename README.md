# minicat

A small cat clone, with pretty printing, inspired by [bat](https://github.com/sharkdp/bat).

Output contents of a file to stdout:
```sh
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
Hello World!
```
