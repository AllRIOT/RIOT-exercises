# name of your application
APPLICATION = shell-example

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../RIOT

# use the shell module
USEMODULE += shell

# Comment this out to disable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
DEVELHELP ?= 1

# Some workarounds are needed in order to get the tutorial running on
# some computers.
-include ../lab_workarounds.mk

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

# Tell the build system to use the Rust crate here
FEATURES_REQUIRED += rust_target
APPLICATION_RUST_MODULE = shell_example
BASELIBS += $(APPLICATION_RUST_MODULE).module

include $(RIOTBASE)/Makefile.include
