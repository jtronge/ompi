# Copyright (c) 2024      Triad National Security, LLC. All rights
#                         reserved.
#
# $COPYRIGHT$
#
# Additional copyrights may follow
#
# $HEADER$
#
import re

FORTRAN_ERROR_NAME = 'ierror'
C_ERROR_NAME = 'ierr'
C_ERROR_TMP_NAME = 'c_ierr'
GENERATED_MESSAGE = 'THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT BY HAND.'
PROTOTYPE_RE = re.compile(
    r"""\.\w+\(                                 # Subroutine name (and opening
                                                # parenthesis);
            (\s*\w+                             # Type name (corresponding to
                                                # classes deriving FortranType below);
             \s+\w+\s*                          # Parameter name;
                (\[\s*\w+\s*=\s*\w+\s*          # Bracket key-value pairs to
                   (;\s*\w+\s*=\s*\w+\s*)*\])?  # type ('[key0=value1;key1=value2...]');
             \s*,?)+                            # Extra whitespace and comma;
                                                # after parameter;
        \)                                      # Closing parenthesis;
        (\s*:\s*(\w+=\w+\s*(;\s*\w+=\w+\s*)*))? # Trailing key-value pairs, to be
                                                # passed to the prototype and type
                                                # handling code;
    """,
    re.X | re.MULTILINE,
)
