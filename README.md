# RIOT in the Internet of Things - Exercises

This is chrysn's version of the exercises, last used for [at FrOSCon](https://programm.froscon.org/2024/events/3228.html).
It contains specific extra examples for portability of Rust code.

Unless this repository has been updated recently, consider looking at [the original exercises from HAW Hamburg's INETRG](https://github.com/inetrg/exercises), which this was forked from.

## Overview

Collection of practical exercises, assignments, and tasks to become familiar
with IoT technologies using Linux and RIOT-OS.

## Getting started

**1. Open a terminal at the local user's home directory**

**2. Clone this repository locally:**
```sh
$ git clone --recurse-submodules https://github.com/inetrg/exercises.git
```

**3. Navigate to the repository and open an editor in the directory:**
```sh
$ cd exercises
$ codium .
```

**4. To avoid installing toolchains, you can use Docker instead. Follow the installation instructions [here](https://doc.riot-os.org/getting-started.html#docker)**

**5. Go through the exercises starting with [`01-hello-world`](./01-hello-world/README.md). Each contains a `README.md` with**
   **detailed instructions and tasks to solve.**

   If you prefer to do the exercises in Rust instead of C, there is a parallel list starting at [`rust01-hello-world`](./rust01-hello-world/README.md).
   The exercises are generally similar, and only differ in details.


## Conventions
Throughout the tutorials, we will specify commands and outputs.
The conventions are as follows:

**Leading `$` means that the command is executed on the linux shell:**
```sh
$ make
```

**Leading `>` means that the command is executed on the RIOT shell:**
```sh
> help
```

**No symbol means an output from the RIOT node on the terminal:**
```sh
Command              Description
---------------------------------------
echo                 Echo a message
```
