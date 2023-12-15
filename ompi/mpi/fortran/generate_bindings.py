from abc import ABC, abstractmethod
import re


class FortranType(ABC):

    def __init__(self, name, **kwargs):
        self.name = name
        self.bigcount = False

    TYPES = {}

    @classmethod
    def add(cls, type_name):
        """Decorator for adding types."""
        def wrapper(class_):
            cls.TYPES[type_name] = class_
            return class_
        return wrapper

    @classmethod
    def get(cls, type_name):
        return cls.TYPES[type_name]

    @abstractmethod
    def declare(self):
        """Return a declaration for the type."""

    def declare_fortran_interface(self):
        """Return the C binding declaration as seen from Fortran."""
        return self.declare()

    def argument(self):
        """Return the value to pass as an argument."""
        return self.name


@FortranType.add('BUFFER')
class BufferType(FortranType):
    def declare(self):
        return f'OMPI_FORTRAN_IGNORE_TKR_TYPE, INTENT(IN) :: {self.name}'


@FortranType.add('COUNT')
class CountType(FortranType):
    def declare(self):
        if self.bigcount:
            return f'INTEGER(KIND=MPI_COUNT_KIND), INTENT(IN) :: {self.name}'
        else:
            return f'INTEGER, INTENT(IN) :: {self.name}'


@FortranType.add('DATATYPE')
class DatatypeType(FortranType):
    def declare(self):
        return f'TYPE(MPI_Datatype), INTENT(IN) :: {self.name}'

    def declare_cbinding_fortran(self):
        return 'INTEGER, INTENT(IN) :: {self.name}'

    def argument(self):
        return f'{self.name}%MPI_VAL'


@FortranType.add('RANK')
class RankType(FortranType):
    def declare(self):
        return f'INTEGER, INTENT(IN) :: {self.name}'


@FortranType.add('TAG')
class TagType(FortranType):
    def declare(self):
        return f'INTEGER, INTENT(IN) :: {self.name}'


@FortranType.add('COMM')
class CommType(FortranType):
    def declare(self):
        return f'TYPE(MPI_Comm), INTENT(IN) :: {self.name}'

    def declare_cbinding_fortran(self):
        return f'INTEGER, INTENT(IN) :: {self.name}'

    def argument(self):
        return f'{self.name}%MPI_VAL'


PROTOTYPE_RE = re.compile(r'^\w+\((\s*\w+\s+\w+\s*,?)+\)$')


class PrototypeParseError(Exception):
    """Thrown when a parsing error is encountered."""


def fortran_f08_name(base_name):
    """Produce the final f08 name from base_name."""
    return f'MPI_{base_name.capitalize()}_f08'


def c_func_name(base_name):
    """Produce the final C func name from base_name."""
    return f'ompi_{base_name}_internal_f08'


def print_header():
    """Print the fortran f08 file header."""
    print('#include "ompi/mpi/fortran/configure-fortran-output.h"')
    print('#include "mpi-f08-rename.h"')


class FortranBinding:

    def __init__(self, fname):
        with open(fname) as fp:
            data = []
            for line in fp:
                data.append(line.strip())
            data = ' '.join(data)
            data = data.strip()
            if PROTOTYPE_RE.match(data) is None:
                raise PrototypeParseError('Invalid function prototype for Fortran interface')
            start = data.index('(')
            end = data.index(')')
            self.fn_name = data[:start].strip()
            parameters = data[start+1:end].split(',')
            self.parameters = []
            for param in parameters:
                param = param.strip()
                type_, name = param.split()
                type_ = FortranType.get(type_)
                indent = '    '
                self.parameters.append(type_(name))

    def _param_list(self):
        return ','.join(type_.name for type_ in self.parameters)

    def _print_fortran_interface(self):
        """Output the C subroutine binding for the Fortran code."""
        name = c_func_name(self.fn_name)
        print('interface')
        print(f'subroutine {name}({self._param_list()},ierror) &')
        print(f'    BIND(C, name="{name}")')
        print('    implicit none')
        for type_ in self.parameters:
            print(f'    {type_.declare_fortran_interface()}')
        print('    INTEGER, INTENT(OUT) :: ierror')
        print(f'end subroutine {name}')
        print('end interface')

    def print_fbinding(self):
        """Output the main MPI Fortran subroutine."""
        print('! THIS FILE WAS AUTOMATICALLY GENERATED. DO NOT EDIT BY HAND.')

        print_header()

        # Interface for call to C function
        self._print_fortran_interface()

        sub_name = fortran_f08_name(self.fn_name)
        c_func = c_func_name(self.fn_name)
        print('subroutine', f'{sub_name}({self._param_list()},ierror)')
        # Parameters/dummy variable declarations
        types = []
        for type_ in self.parameters:
            print(f'    {type_.declare()}')
        # Add the integer error manually
        print('    INTEGER, OPTIONAL, INTENT(OUT) :: ierror')
        # Temporaries
        print('    INTEGER :: c_ierror')

        # Call into the C function
        args = ','.join(type_.argument() for type_ in self.parameters)
        print(f'    call {c_func}({args},c_ierror)')
        # Convert error type
        print('    if (present(ierror)) ierror = c_ierror')

        print(f'end subroutine {sub_name}')


FortranBinding('use-mpi-f08/send.in').print_fbinding()
