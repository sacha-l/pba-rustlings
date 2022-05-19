Note: please don't publish the solutions to the exercises!

# Rustlings for the Polkadot Academy Self-Assessment 

This is a curated, shorter version of the original [Rustlings](https://github.com/rust-lang/rustlings/) exercises.
It covers most basic topics to get started with programming with Rust with a couple of more challenging exercises with fewer hints than in the Rustlings repository.

The purpose of this repository is to serve as a faster way to assess your Rust skills in order to be prepared for the Polkadot Blockchain Academy. 
It does assume that users have experience with Rust and is not meant to test their ability to learn, but rather to refresh on some basic Rust.

It contains 42 exercises, covering the basics of:

- Variables, primitive types, standard library types
- Functions, structs and traits
- Modules
- Error handling
- Generics, options and traits
- Macros
- Type conversions

# Instructions

1. Clone this repo locally.
1. Read through the  [Rust by Example book](https://doc.rust-lang.org/rust-by-example/hello/print.html), the exercises are based on things there.
1. Follow the original [Getting Started](#getting-started) instructions to install Rustlings.
1. In the root directory, hit `rustlings watch`.
1. Complete the exercises! Type `hint` if you need help.
1. Save a screenshot of your terminal when all compilation and tests have passed.

# rustlings 🦀❤️

Greetings and welcome to `rustlings`. This project contains small exercises to get you used to reading and writing Rust code. This includes reading and responding to compiler messages!

_...looking for the old, web-based version of Rustlings? Try [here](https://github.com/rust-lang/rustlings/tree/rustlings-1)_

Alternatively, for a first-time Rust learner, there are several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes. You will be using this along with Rustlings!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online

## Getting Started

_Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`._
_Note: If you're on Linux, make sure you've installed gcc. Deb: `sudo apt install gcc`. Yum: `sudo yum -y install gcc`._

You will need to have Rust installed. You can get it by visiting https://rustup.rs. This'll also install Cargo, Rust's package/project manager.

## MacOS/Linux

Just run:

```bash
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash
# Or if you want it to be installed to a different path:
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash -s mypath/
```

This will install Rustlings and give you access to the `rustlings` command. Run it to get started!

## Windows

In PowerShell (Run as Administrator), set `ExecutionPolicy` to `RemoteSigned`:

```ps1
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

Then, you can run:

```ps1
Start-BitsTransfer -Source https://raw.githubusercontent.com/rust-lang/rustlings/main/install.ps1 -Destination $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

To install Rustlings. Same as on MacOS/Linux, you will have access to the `rustlings` command after it.

When you get a permission denied message then you have to exclude the directory where you placed the rustlings in your virus-scanner

## Browser:

[Run on Repl.it](https://repl.it/github/rust-lang/rustlings)

[![Open in Gitpod](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/rust-lang/rustlings)

## Manually

Basically: Clone the repository at the latest tag, run `cargo install`.

```bash
# find out the latest version at https://github.com/rust-lang/rustlings/releases/latest (on edit 4.7.1)
git clone -b 4.7.1 --depth 1 https://github.com/rust-lang/rustlings
cd rustlings
cargo install --force --path .
```

If there are installation errors, ensure that your toolchain is up to date. For the latest, run:

```bash
rustup update
```

Then, same as above, run `rustlings` to get started.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start.

The task is simple. Most exercises contain an error that keeps them from compiling, and it's up to you to fix it! Some exercises are also run as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings watch
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). It will also rerun automatically every time you change a file in the `exercises/` directory. If you want to only run it once, you can use:

```bash
rustlings verify
```

This will do the same as watch, but it'll quit after running.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run myExercise1
```

Or simply use the following command to run the next unsolved exercise in the course:

```bash
rustlings run next
```

In case you get stuck, you can run the following command to get a hint for your
exercise:

```bash
rustlings hint myExercise1
```

You can also get the hint for the next unsolved exercise with the following command:

```bash
rustlings hint next
```

To check your progress, you can run the following command:

```bash
rustlings list
```

## Testing yourself

After every couple of sections, there will be a quiz that'll test your knowledge on a bunch of sections at once. These quizzes are found in `exercises/quizN.rs`.

## Enabling `rust-analyzer`

`rust-analyzer` support is provided, but it depends on your editor
whether it's enabled by default. (RLS support is not provided)

To enable `rust-analyzer`, you'll need to make Cargo build the project
with the `exercises` feature, which will automatically include the `exercises/`
subfolder in the project. The easiest way to do this is to tell your editor to
build the project with all features (the equivalent of `cargo build --all-features`).
For specific editor instructions:

- **VSCode**: Add a `.vscode/settings.json` file with the following:
```json
{
    "rust-analyzer.cargo.features": ["exercises"]
}
```
- **IntelliJ-based Editors**: Using the Rust plugin, everything should work
    by default.
- _Missing your editor? Feel free to contribute more instructions!_

## Continuing On

Once you've completed Rustlings, put your new knowledge to good use! Continue practicing your Rust skills by building your own projects, contributing to Rustlings, or finding other open-source projects to contribute to.

## Uninstalling Rustlings

If you want to remove Rustlings from your system, there's two steps. First, you'll need to remove the exercises folder that the install script created
for you:

```bash
rm -rf rustlings # or your custom folder name, if you chose and or renamed it
```

Second, since Rustlings got installed via `cargo install`, it's only reasonable to assume that you can also remove it using Cargo, and
exactly that is the case. Run `cargo uninstall` to remove the `rustlings` binary:

```bash
cargo uninstall rustlings
```

Now you should be done!

## Completion

Rustlings isn't done; there are a couple of sections that are very experimental and don't have proper documentation. These include:

- Errors (`exercises/errors/`)
- Option (`exercises/option/`)
- Result (`exercises/result/`)
- Move Semantics (could still be improved, `exercises/move_semantics/`)

Additionally, we could use exercises on a couple of topics:

- Structs
- Better ownership stuff
- `impl`
- ??? probably more

If you are interested in improving or adding new ones, please feel free to contribute! Read on for more information :)

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md).

## Contributors ✨

Thanks goes to the wonderful people listed in [AUTHORS.md](./AUTHORS.md) 🎉
