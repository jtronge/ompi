/*
 * Copyright (c) 2023 Triad National Security, LLC. All rights reserved.
 * $COPYRIGHT$
 *
 * Additional copyrights may follow
 *
 * $HEADER$
 */

#include "ompi_config.h"

#include <stdlib.h>

#include "opal/util/show_help.h"
#include "ompi/runtime/ompi_spc.h"
#include "ompi/mpi/c/bindings.h"
#include "ompi/communicator/communicator.h"
#include "ompi/errhandler/errhandler.h"
#include "ompi/constants.h"

int MPI_Abi_supported(int *flag)
{
    *flag = 1;
    return MPI_SUCCESS;
}

int MPI_Abi_version(int *abi_major, int *abi_minor)
{
    /* 0.1 */
    *abi_major = 0;
    *abi_minor = 1;
    return MPI_SUCCESS;
}

static const char ABI_DETAILS[] = "Open MPI Standard ABI 0.1";

int MPI_Abi_details(int *buflen, char *details, MPI_Info *info)
{
    if (*buflen >= sizeof(ABI_DETAILS)) {
        strcpy(details, ABI_DETAILS);
        *buflen = sizeof(ABI_DETAILS);
        return MPI_SUCCESS;
    } else {
        *buflen = 0;
        return MPI_ERR_BUFFER;
    }
}
