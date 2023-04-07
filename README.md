# Tudo - CLI To-Do List Tool

<p align="center">
  <img width="256" height="256" src="https://github.com/rmrz-daniel/Tudo/blob/main/src/Photos/tudo.png">
</p>

Tudo is a fast and simple CLI tool for managing to-do lists. With Tudo, you can easily add, remove, clear, and mark tasks as done, all from the command line. Tudo is written in Rust, making it a BLAZINGLY high-performance and efficient tool for staying organized.

## Installation

To use Tudo, you can install Tudo using Cargo, Rust's package manager:

```
cargo install tudo
```

alternatively you may install the executable from the release tab and manual add the .exe to your win11 path env

<p align="center">
  <img src="https://github.com/rmrz-daniel/Tudo/blob/main/src/Photos/Path.png">
</p>

## Usage

To view the help message and available commands, use the `tudo` option by itself:

```
tudo
```

To initialize tudo at a project use the `init` option at the wanted directory:

```
tudo init
```


To add a task from your to-do list, use the `add` option followed by a task or multiple tasks:

```
tudo add "Buy groceries"
tudo add "Buy groceries" "Finish project"
```

To remove a task from your to-do list, use the `remove` option to open a visual prompt or use `remove` option followed by a task or multiple tasks:

```
tudo remove
tudo remove bug2300
tudo remove "Pick up groceries" "Finish project"
```

To mark a task as done, use the `toggle` option to open prompt and select a task:

```
tudo toggle
```

To clear all tasks from your to-do list, use the `clear` option:

```
tudo clear
```
