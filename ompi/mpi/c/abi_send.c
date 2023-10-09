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

#include "ompi/mpi/c/bindings.h"
#include "ompi/runtime/params.h"
#include "ompi/communicator/communicator.h"
#include "ompi/errhandler/errhandler.h"
#include "ompi/mca/pml/pml.h"
#include "ompi/datatype/ompi_datatype.h"
#include "ompi/mpi/c/abi.h"

int MPI_Send(const void *buf, int count, MPI_Datatype type, int dest,
             int tag, MPI_Comm comm)
{
    int rc = MPI_SUCCESS;
    ompi_datatype_t *ompi_type;
    ompi_communicator_t *ompi_comm;

    ompi_type = ompi_abi_datatype_convert_internal(type);
    ompi_comm = ompi_abi_comm_convert_internal(comm);

    if (MPI_PARAM_CHECK) {
        if (NULL == ompi_comm) {
            return OMPI_ERRHANDLER_NOHANDLE_INVOKE(MPI_ERR_COMM, __func__);
        } else if (count < 0) {
            rc = MPI_ERR_COUNT;
        } else if (tag < 0 || tag > mca_pml.pml_max_tag) {
            rc = MPI_ERR_TAG;
        } else if (ompi_comm_peer_invalid(ompi_comm, dest)
                   && MPI_PROC_NULL != dest) {
            rc = MPI_ERR_RANK;
        } else {
            OMPI_CHECK_DATATYPE_FOR_SEND(rc, ompi_type, count);
            OMPI_CHECK_USER_BUFFER(rc, buf, ompi_type, count);
        }
        OMPI_ERRHANDLER_CHECK(rc, ompi_comm, rc, __func__);
    }

    if (MPI_PROC_NULL == dest) {
        return MPI_SUCCESS;
    }

    rc = MCA_PML_CALL(send(buf, count, ompi_type, dest, tag, MCA_PML_BASE_SEND_STANDARD, ompi_comm));
    OMPI_ERRHANDLER_RETURN(rc, ompi_comm, rc, __func__);
}
