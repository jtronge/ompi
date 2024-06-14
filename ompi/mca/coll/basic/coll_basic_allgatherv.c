/*
 * Copyright (c) 2004-2005 The Trustees of Indiana University and Indiana
 *                         University Research and Technology
 *                         Corporation.  All rights reserved.
 * Copyright (c) 2004-2017 The University of Tennessee and The University
 *                         of Tennessee Research Foundation.  All rights
 *                         reserved.
 * Copyright (c) 2004-2005 High Performance Computing Center Stuttgart,
 *                         University of Stuttgart.  All rights reserved.
 * Copyright (c) 2004-2005 The Regents of the University of California.
 *                         All rights reserved.
 * Copyright (c) 2012      Oak Ridge National Labs.  All rights reserved.
 * Copyright (c) 2015      Research Organization for Information Science
 *                         and Technology (RIST). All rights reserved.
 * $COPYRIGHT$
 *
 * Additional copyrights may follow
 *
 * $HEADER$
 */

#include "ompi_config.h"
#include "coll_basic.h"

#include "mpi.h"
#include "ompi/datatype/ompi_datatype.h"
#include "ompi/constants.h"
#include "ompi/mca/coll/coll.h"
#include "ompi/mca/coll/base/coll_tags.h"
#include "coll_basic.h"


/*
 *	allgatherv_inter
 *
 *	Function:	- allgatherv using other MPI collectives
 *	Accepts:	- same as MPI_Allgatherv()
 *	Returns:	- MPI_SUCCESS or error code
 */
int
mca_coll_basic_allgatherv_inter(const void *sbuf, size_t scount,
                                struct ompi_datatype_t *sdtype,
                                void *rbuf, ompi_count_array *rcounts, ompi_disp_array *disps,
                                struct ompi_datatype_t *rdtype,
                                struct ompi_communicator_t *comm,
                                mca_coll_base_module_t *module)
{
    int rsize, err, i;
    size_t *scounts;
    ptrdiff_t *sdisps;
    ompi_count_array scounts_desc;
    ompi_disp_array sdisps_desc;

    rsize = ompi_comm_remote_size(comm);

    scounts = (size_t *) malloc(rsize * sizeof(size_t) + rsize * sizeof(ptrdiff_t));
    sdisps = (ptrdiff_t *) (scounts + rsize);
    if (NULL == scounts) {
        return OMPI_ERR_OUT_OF_RESOURCE;
    }

    for (i = 0; i < rsize; i++) {
        scounts[i] = scount;
        sdisps[i] = 0;
    }

    scounts_desc.type = OMPI_COUNT_ARRAY_TYPE_SIZE_T;
    scounts_desc.data.size_t_array = scounts;
    sdisps_desc.type = OMPI_DISP_ARRAY_TYPE_PTRDIFF_T;
    sdisps_desc.data.ptrdiff_t_array = sdisps;
    err = comm->c_coll->coll_alltoallv(sbuf, &scounts_desc, &sdisps_desc, sdtype,
                                      rbuf, rcounts, disps, rdtype, comm,
                                      comm->c_coll->coll_alltoallv_module);

    if (NULL != scounts) {
        free(scounts);
    }

    return err;
}
