# Copyright (c) 2024      Triad National Security, LLC. All rights
#                         reserved.
#
# $COPYRIGHT$
#
# Additional copyrights may follow
#
# $HEADER$
import argparse
import os
import sys


def main():
    parser = argparse.ArgumentParser(description='generate fortran binding files')
    parser.add_argument('--template', required=True, help='template file to use')
    parser.add_argument('--output', required=True, help='output file to use')
    parser.add_argument('--builddir', required=True, help='absolute path to automake builddir (abs_top_builddir)')
    parser.add_argument('--srcdir', required=True, help='absolute path to automake srcdir (abs_top_srcdir)')
    subparsers = parser.add_subparsers()

    # Handler for generating actual code
    parser_code = subparsers.add_parser('code', help='generate binding code')
    parser_code.add_argument('lang', choices=('fortran', 'c'),
                             help='generate dependent files in C or Fortran')
    parser_code.set_defaults(handler=lambda *pargs, **kwargs: generate_code(*pargs, **kwargs))

    # Handler for generating the Fortran interface files
    parser_interface = subparsers.add_parser('interface',
                                             help='generate Fortran interface specifcations')
    parser_interface.set_defaults(handler=lambda *pargs, **kwargs: generate_interface(*pargs, **kwargs))

    args = parser.parse_args()

    # Below lines are required in order to pull in both generated python files and src files
    sys.path.insert(0, os.path.join(args.builddir, 'ompi/mpi/bindings'))
    sys.path.insert(0, os.path.join(args.srcdir, 'ompi/mpi/bindings'))
    from ompi_bindings.fortran import generate_code, generate_interface, load_prototypes
    from ompi_bindings.util import OutputFile

    prototypes = load_prototypes(args.template)
    with open(args.output, 'w') as f:
        args.handler(args, prototypes, OutputFile(f))


if __name__ == '__main__':
    main()
