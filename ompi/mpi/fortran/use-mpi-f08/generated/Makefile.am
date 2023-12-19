# -*- makefile.am -*-
#
# Copyright (c) 2023      Triad National Security, LLC. All rights
#                         reserved.
#
# $COPYRIGHT$
#
# Additional copyrights may follow
#
# $HEADER$
#

# This Makefile is only relevant if we're building the "use mpi_f08"
# MPI bindings.
if OMPI_BUILD_FORTRAN_USEMPIF08_BINDINGS

noinst_LTLIBRARIES = libmpi_usempif08_generated.la

generated_fortran = \
        send_f08_generated.F90

if OMPI_GENERATE_BINDINGS
%_generated.F90: %.in generate_bindings.py
        $(PYTHON) $(srcdir)/generate_bindings.py fortran $< > $@

MAINTAINERCLEANFILES += $(generated_fortran)
endif

generated_c = \
        send_f08_generated.c

if OMPI_GENERATE_BINDINGS
%_generated.c: ../%.in $(srcdir)/generate_bindings.py
	$(PYTHON) $(srcdir)/generate_bindings.py c $< > $@

MAINTAINERCLEANFILES += $(generated_c)
endif

nodist_libmpi_usempif08_generated_la_SOURCES = \
        $(generated_fortran) \
        $(generated_c)

endif # OMPI_BUILD_FORTRAN_USEMPIF08_BINDINGS
