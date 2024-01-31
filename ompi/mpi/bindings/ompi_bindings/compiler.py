# Copyright (c) 2024      Triad National Security, LLC. All rights
#                         reserved.
#
# $COPYRIGHT$
#
# Additional copyrights may follow
#
# $HEADER$

# Check if we have support for TS 29113 (templated at configure time)
HAVE_TS = '0' == '1'
OMPI_F08_IGNORE_TKR_PREDECL = '!GCC$ ATTRIBUTES NO_ARG_CHECK ::'
OMPI_F08_IGNORE_TKR_TYPE = 'type(*), dimension(*)'
OMPI_FORTRAN_IGNORE_TKR_PREDECL = '!GCC$ ATTRIBUTES NO_ARG_CHECK ::'
OMPI_FORTRAN_IGNORE_TKR_TYPE = 'type(*), dimension(*)'
