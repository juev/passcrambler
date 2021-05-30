# Password scrambler

Fork [password_scrambler](https://github.com/hasherezade/password_scrambler).

Small utility to generate complicated passwords.

## Benefits

+ You get **more secure password**- long, not from dictionary, etc i.e
  'txork9Zfa8yXc_lMbb1LCHPZIH7wE1'

+ Yet, **you don't have to remeber it** - you must remember only your easy
  password and document that you used as a generation base

+ You **may reuse** the easy password and the base file - still, for different
  login@domain you will get a totally new long password

+ It is not saving your complicated password anywhere, so nobody can steal it
  and decrypt - it generates it by hasing function and you just need to copy it
  and login where you want

+ Open source, written in rust - nothing is hidden under the hood, **everyone
  can review it before using** and make custom changes in code

## How it works

Password scrambler will generate the same passwords on content of base file and
master password.

You can specify password length, file and symbols on alias:

```bash
alias passcrambler=passcrambler -f ~/.file -L 20 -s '_$'
```

All files should be generate with the same options. This needed only for
remmembering.

## Help

```bash
passcrambler: v.0.1.0

USAGE:
  passcrambler [OPTIONS]

FLAGS:
  -h, --help            Prints help information
  -c, --clip            Copy the generated password into the clipboard instead of displaying

OPTIONS:
  -f, --file FILE       File for seeding password, REQUIRED
  -l, --login LOGIN     Login data for password, REQUIRED
  -L, --length 30       Length of the password, default=30
  -s, --symbols '_&#'   Symbols for using in password, default='_-&#*^%$@!~'
```

## Example

```bash
❯ passcrambler --file Cargo.toml --login denis@google
Type password: _12345_
---
C@4%0PGx*JfdN3AyaSN4K5hTUDlN4^
```

## Install

You can download binary distribution from [Release
page](https://github.com/juev/passcrambler/releases/latest)

or you can install with cargo:

```bash
cargo install --git https://github.com/juev/passcrambler --branch main
```

## Typical scenario

+ I need to generate a new password i.e. for my e-mail

+ I have to prepare 2 things : an easy password, that I will remember and some
  document, that I have to keep safe without changes

+ I deploy password scrambler giving as an input my login and a document

+ I am prompted for the easy password, so I type it

+ I copy generated password and change it in my e-mail service

+ Wherever I need to re-login I just deploy scrambler with same parameters, and
  it will regenerate the same hash
