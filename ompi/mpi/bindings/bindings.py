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


def handle_missing_command(args, out):
    print('missing subcommand (one of {fortran,c} required)', file=sys.stderr)
    sys.exit(1)


def main():
    parser = argparse.ArgumentParser(description='generate fortran binding files')
    parser.add_argument('--builddir', required=True, help='absolute path to automake builddir (abs_top_builddir)')
    parser.add_argument('--output', required=True, help='output file to use')
    parser.add_argument('--srcdir', required=True, help='absolute path to automake srcdir (abs_top_srcdir)')
    parser.set_defaults(handler=handle_missing_command)
    subparsers = parser.add_subparsers()

    # Fortran set up code
    parser_fortran = subparsers.add_parser('fortran', help='subcommand for generating Fortran code')
    parser_fortran.add_argument('--template', required=True, help='template file to use')
    subparsers_fortran = parser_fortran.add_subparsers()
    # Handler for generating actual code
    parser_code = subparsers_fortran.add_parser('code', help='generate binding code')
    parser_code.add_argument('lang', choices=('fortran', 'c'),
                             help='generate dependent files in C or Fortran')
    parser_code.set_defaults(handler=lambda args, out: fortran.generate_code(args, out))
    # Handler for generating the Fortran interface files
    parser_interface = subparsers_fortran.add_parser('interface',
                                                     help='generate Fortran interface specifcations')
    parser_interface.set_defaults(handler=lambda args, out: fortran.generate_interface(args, out))

    # C set up code
    parser_c = subparsers.add_parser('c', help='subcommand for generating C code')
    subparsers_c = parser_c.add_subparsers()
    parser_header = subparsers_c.add_parser('header')
    parser_header.add_argument('file', nargs='+', help='list of template source files')
    parser_header.add_argument('--external', action='store_true', help='generate external mpi.h header file')
    parser_header.add_argument('--srcdir', help='source directory')
    parser_header.set_defaults(handler=lambda args, out: c.generate_header(args, out))
    parser_gen = subparsers_c.add_parser('source')
    # parser = argparse.ArgumentParser(description='C ABI binding generation code')
    parser_gen.add_argument('type', choices=('ompi', 'standard'),
                            help='generate the OMPI ABI functions or the standard ABI functions')
    parser_gen.add_argument('source_file', help='template file to use for C code generation')
    parser_gen.set_defaults(handler=lambda args, out: c.generate_source(args, out))
    args = parser.parse_args()

    # Pull in both generated python files and src files on import
    sys.path.insert(0, os.path.join(args.builddir, 'ompi/mpi/bindings'))
    sys.path.insert(0, os.path.join(args.srcdir, 'ompi/mpi/bindings'))
    from ompi_bindings import c, fortran
    from ompi_bindings.util import OutputFile

    with open(args.output, 'w') as f:
        args.handler(args, OutputFile(f))


if __name__ == '__main__':
    main()
