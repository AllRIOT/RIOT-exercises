# name of your application
APPLICATION = sock

# If no BOARD is found in the environment, use this default:
BOARD ?= native

# This has to be the absolute path to the RIOT base directory:
RIOTBASE ?= $(CURDIR)/../RIOT

# enable default networking devides in the platform
USEMODULE += netdev_default

# automatically initialize network interfaces for enabled devices
USEMODULE += auto_init_gnrc_netif

# enable single interface optimization.
# remove this if more than one interface is present
USEMODULE += gnrc_netif_single

# add minimal IPv6 support
USEMODULE += gnrc_ipv6_default

# enable ping support
USEMODULE += gnrc_icmpv6_echo

# add UDP sock support
USEMODULE += sock_udp

# add event support for sock
USEMODULE += sock_async_event

# Helper functions for sock
USEMODULE += sock_util

# add also the shell, some shell commands
USEMODULE += shell
USEMODULE += shell_cmds_default

# Comment this out to disable code in RIOT that does safety checking
# which is not needed in a production environment but helps in the
# development process:
DEVELHELP ?= 1

# Some workarounds are needed in order to get the tutorial running on
# some computers.
-include ../lab_workarounds.mk

# Change this to 0 show compiler invocation lines by default:
QUIET ?= 1

include $(RIOTBASE)/Makefile.include
