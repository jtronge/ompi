/*
 * Copyright (c) 2024 Triad National Security, LLC. All rights reserved.
 * $COPYRIGHT$
 *
 * Additional copyrights may follow
 *
 * $HEADERS$
 *
 * Macros used by the generate_bindings.py.in script when generating the C
 * wrapper code.
 */

#define OMPI_VBUFFER_PREPARE(name, tmp_name, datatype, tmp_datatype, comm, counts, displs, ierr, c_ierr, fn_name) do { \
    if (OMPI_COMM_IS_INTER(comm) || !OMPI_IS_FORTRAN_IN_PLACE(tmp_name)) { \
        tmp_datatype = PMPI_Type_f2c(*datatype); \
        OMPI_CFI_CHECK_CONTIGUOUS(name, c_ierr); \
        if (MPI_SUCCESS != c_ierr) { \
            *ierr = OMPI_INT_2_FINT(c_ierr); \
            OMPI_ERRHANDLER_INVOKE(comm, c_ierr, fn_name) \
            return; \
        } \
        OMPI_ARRAY_FINT_2_INT(counts, size); \
        OMPI_ARRAY_FINT_2_INT(displs, size); \
    } else { \
        tmp_name = MPI_IN_PLACE; \
    } \
} while (0)

#define OMPI_VBUFFER_OUT_PREPARE(name, tmp_name, comm, ierr, c_ierr, fn_name) do { \
    OMPI_CFI_CHECK_CONTIGUOUS(name, c_ierr); \
    if (MPI_SUCCESS != c_ierr) { \
        *ierr = OMPI_INT_2_FINT(c_ierr); \
        OMPI_ERRHANDLER_INVOKE(comm, c_ierr, fn_name) \
        return; \
    } \
} while (0)
