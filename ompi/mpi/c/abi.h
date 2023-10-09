#ifndef _OMPI_ABI_H_
#define _OMPI_ABI_H_

#include "ompi/mpi/c/bindings.h"
#include "ompi/communicator/communicator.h"
#include "ompi/datatype/ompi_datatype.h"

static inline ompi_communicator_t *ompi_abi_comm_convert_internal(MPI_Comm comm)
{
    if (NULL == comm || MPI_COMM_NULL == comm) {
        return NULL;
    } else if (MPI_COMM_WORLD == comm) {
        return (ompi_communicator_t *) &ompi_mpi_comm_world;
    } else if (MPI_COMM_SELF == comm) {
        return (ompi_communicator_t *) &ompi_mpi_comm_self;
    }

    return (ompi_communicator_t *) comm;
}

static inline ompi_datatype_t *ompi_abi_datatype_convert_internal(MPI_Datatype type)
{
    if (MPI_DATATYPE_NULL == type) {
        return (ompi_datatype_t *) &ompi_mpi_datatype_null;
    } else if (MPI_BYTE == type) {
        return (ompi_datatype_t *) &ompi_mpi_byte;
    } else if (MPI_PACKED == type) {
        return (ompi_datatype_t *) &ompi_mpi_packed;
    } else if (MPI_CHAR == type) {
        return (ompi_datatype_t *) &ompi_mpi_char;
    } else if (MPI_SHORT == type) {
        return (ompi_datatype_t *) &ompi_mpi_short;
    } else if (MPI_INT == type) {
        return (ompi_datatype_t *) &ompi_mpi_int;
    } else if (MPI_LONG == type) {
        return (ompi_datatype_t *) &ompi_mpi_long;
    } else if (MPI_FLOAT == type) {
        return (ompi_datatype_t *) &ompi_mpi_float;
    } else if (MPI_DOUBLE == type) {
        return (ompi_datatype_t *) &ompi_mpi_double;
    } else if (MPI_LONG_DOUBLE == type) {
        return (ompi_datatype_t *) &ompi_mpi_long_double;
    } else if (MPI_SIGNED_CHAR == type) {
        return (ompi_datatype_t *) &ompi_mpi_signed_char;
    } else if (MPI_UNSIGNED_SHORT == type) {
        return (ompi_datatype_t *) &ompi_mpi_unsigned_short;
    } else if (MPI_UNSIGNED_LONG == type) {
        return (ompi_datatype_t *) &ompi_mpi_unsigned_long;
    } else if (MPI_UNSIGNED == type) {
        return (ompi_datatype_t *) &ompi_mpi_unsigned;
    } else if (MPI_FLOAT_INT == type) {
        return (ompi_datatype_t *) &ompi_mpi_float_int;
    } else if (MPI_DOUBLE_INT == type) {
        return (ompi_datatype_t *) &ompi_mpi_double_int;
    } else if (MPI_LONG_DOUBLE_INT == type) {
        return (ompi_datatype_t *) &ompi_mpi_longdbl_int;
    } else if (MPI_LONG_INT == type) {
        return (ompi_datatype_t *) &ompi_mpi_long_int;
    } else if (MPI_SHORT_INT == type) {
        return (ompi_datatype_t *) &ompi_mpi_short_int;
    } else if (MPI_2INT == type) {
        return (ompi_datatype_t *) &ompi_mpi_2int;
    } else if (MPI_INT8_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_int8_t;
    } else if (MPI_UINT8_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_uint8_t;
    } else if (MPI_INT16_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_int16_t;
    } else if (MPI_UINT16_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_uint16_t;
    } else if (MPI_INT32_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_int32_t;
    } else if (MPI_UINT32_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_uint32_t;
    } else if (MPI_INT64_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_int64_t;
    } else if (MPI_UINT64_T == type) {
        return (ompi_datatype_t *) &ompi_mpi_uint64_t;
    } else if (MPI_OFFSET == type) {
        return (ompi_datatype_t *) &ompi_mpi_offset;
    } else if (MPI_C_BOOL == type) {
        return (ompi_datatype_t *) &ompi_mpi_c_bool;
#if HAVE_FLOAT__COMPLEX
    } else if (MPI_C_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_c_float_complex;
    } else if (MPI_C_FLOAT_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_c_float_complex;
#endif
#if HAVE_DOUBLE__COMPLEX
    } else if (MPI_C_DOUBLE_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_c_double_complex;
#endif
#if HAVE_LONG_DOUBLE__COMPLEX
    } else if (MPI_C_LONG_DOUBLE_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_c_long_double_complex;
#endif
    } else if (MPI_CXX_BOOL == type) {
        return (ompi_datatype_t *) &ompi_mpi_cxx_bool;
    } else if (MPI_CXX_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_cxx_cplex;
    } else if (MPI_CXX_FLOAT_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_cxx_cplex;
    } else if (MPI_CXX_DOUBLE_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_cxx_dblcplex;
    } else if (MPI_CXX_LONG_DOUBLE_COMPLEX == type) {
        return (ompi_datatype_t *) &ompi_mpi_cxx_ldblcplex;
    } else if (MPI_COUNT == type) {
        return (ompi_datatype_t *) &ompi_mpi_count;
    } else {
        return (ompi_datatype_t *) type;
    }
}

#endif /* _OMPI_ABI_H_ */
