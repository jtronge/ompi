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

/* #include "ompi/include/mpi_abi/mpi.h" */
#include "ompi/communicator/communicator.h"

extern struct ompi_predefined_communicator_t ompi_mpi_comm_world;
extern struct ompi_predefined_communicator_t ompi_mpi_comm_self;
extern struct ompi_predefined_communicator_t ompi_mpi_comm_null;

int MPI_Comm_rank(MPI_Comm comm, int *rank)
{
    ompi_communicator_t *ompi_comm;

    if (MPI_COMM_WORLD == comm) {
        ompi_comm = (ompi_communicator_t *) &ompi_mpi_comm_world;
    } else if (MPI_COMM_SELF == comm) {
        ompi_comm = (ompi_communicator_t *) &ompi_mpi_comm_self;
    } else if (MPI_COMM_NULL == comm) {
        ompi_comm = (ompi_communicator_t *) &ompi_mpi_comm_null;
    } else {
        ompi_comm = (ompi_communicator_t *) comm;
    }

    *rank = ompi_comm_rank(ompi_comm);
    return MPI_SUCCESS;
}
