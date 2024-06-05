# RIOT in the Internet of Things - Exercises

This is chrysn's version of the exercises, last used for [the RIOT summit](https://summit.riot-os.org/2024/).

Unless this repository has been updated recently, consider looking at [the original exercises from HAW Hamburg's INETRG](https://github.com/inetrg/exercises) or [the TU Dresden version](https://github.com/netd-tud/WSN-exercises), which this was derived from.

## Overview

Collection of practical exercises, assignments, and tasks to become familiar
with IoT technologies using Linux and RIOT-OS.

This is a version of https://github.com/inetrg/exercises
adapted for the course "Wireless Sensor Networks" at TU Dresden.

## Getting started

Go through the exercises starting with [`00-getting-started`](./00-getting-started/README.md).
Each exercise contains a `README.md` with detailed instructions and tasks to solve.

   If you prefer to do the exercises in Rust instead of C, there is a parallel list starting at [`rust01-hello-world`](./rust01-hello-world/README.md).
   The exercises are generally similar, and only differ in details.


## Conventions
Throughout the tutorials, we will specify commands and outputs.
The conventions are as follows:

**Leading `$` means that the command is executed on the Linux shell:**
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
