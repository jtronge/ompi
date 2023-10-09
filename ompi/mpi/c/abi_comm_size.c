/*
 * Copyright (c) 2023 Triad National Security, LLC. All rights reserved.
 * $COPYRIGHT$
 *
 * Additional copyrights may follow
 *
 * $HEADER$
 */
#include "ompi_config.h"
#include <stdio.h>

#include "ompi/communicator/communicator.h"
#include "ompi/errhandler/errhandler.h"
#include "ompi/mpi/c/abi.h"

int MPI_Comm_size(MPI_Comm comm, int *size)
{
    ompi_communicator_t *ompi_comm;

    ompi_comm = ompi_abi_comm_convert_internal(comm);

    if (MPI_PARAM_CHECK) {
        if (NULL == ompi_comm) {
            return OMPI_ERRHANDLER_NOHANDLE_INVOKE(MPI_ERR_COMM, __func__);
        }
        if (NULL == size) {
            return OMPI_ERRHANDLER_INVOKE(ompi_comm, MPI_ERR_ARG, __func__);
        }
    }

    *size = ompi_comm_size(ompi_comm);
    return MPI_SUCCESS;
}
