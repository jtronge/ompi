# Copyright (c) 2024      Triad National Security, LLC. All rights
#                         reserved.
#
# $COPYRIGHT$
#
# Additional copyrights may follow
#
# $HEADER$
"""Utility code for OMPI binding generation."""

class OutputFile:
    """Output file of script."""

    def __init__(self, fp):
        self.fp = fp

    def dump(self, *pargs, **kwargs):
        print(*pargs, **kwargs, file=self.fp)
