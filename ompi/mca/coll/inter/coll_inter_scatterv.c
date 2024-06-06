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
 * Copyright (c) 2006-2010 University of Houston. All rights reserved.
 * Copyright (c) 2015-2016 Research Organization for Information Science
 *                         and Technology (RIST). All rights reserved.
 * Copyright (c) 2022      IBM Corporation.  All rights reserved.
 * $COPYRIGHT$
 *
 * Additional copyrights may follow
 *
 * $HEADER$
 */

#include "ompi_config.h"
#include "coll_inter.h"

#include "mpi.h"
#include "ompi/constants.h"
#include "ompi/datatype/ompi_datatype.h"
#include "ompi/mca/coll/coll.h"
#include "ompi/mca/coll/base/coll_tags.h"
#include "ompi/mca/pml/pml.h"


/*
 *	scatterv_inter
 *
 *	Function:	- scatterv operation
 *	Accepts:	- same arguments as MPI_Scatterv()
 *	Returns:	- MPI_SUCCESS or error code
 */
int
mca_coll_inter_scatterv_inter(const void *sbuf, ompi_count_array *scounts,
                              ompi_disp_array *disps, struct ompi_datatype_t *sdtype,
                              void *rbuf, size_t rcount,
                              struct ompi_datatype_t *rdtype, int root,
                              struct ompi_communicator_t *comm,
                              mca_coll_base_module_t *module)
{
    int i, rank, size, err, size_local;
    size_t total = 0;
    size_t *counts=NULL;
    ptrdiff_t *displace=NULL;
    ompi_count_array counts_arg;
    ompi_disp_array displace_arg;
    char *ptmp_free=NULL, *ptmp=NULL;
    ompi_datatype_t *ndtype;

    /* Initialize */

    rank = ompi_comm_rank(comm);
    size = ompi_comm_remote_size(comm);
    size_local = ompi_comm_size(comm);

    if (MPI_PROC_NULL == root) {
        /* do nothing */
        err = OMPI_SUCCESS;
    } else if (MPI_ROOT != root) {
	if(0 == rank) {
	    /* local root receives the counts from the root */
	    counts = (size_t *)malloc(sizeof(size_t) * size_local);
	    err = MCA_PML_CALL(recv(counts, sizeof(size_t) * size_local, MPI_BYTE,
				    root, MCA_COLL_BASE_TAG_SCATTERV,
				    comm, MPI_STATUS_IGNORE));
	    if (OMPI_SUCCESS != err) {
		return err;
	    }
	    /* calculate the whole buffer size and receive it from root */
	    for (i = 0; i < size_local; i++) {
		total = total + counts[i];
	    }
	    if ( total > 0 ) {
                ptrdiff_t gap, span;
                span = opal_datatype_span(&rdtype->super, total, &gap);
		ptmp_free = (char*)malloc(span);
		if (NULL == ptmp_free) {
		    return OMPI_ERR_OUT_OF_RESOURCE;
		}
                ptmp = ptmp_free - gap;
	    }
	    err = MCA_PML_CALL(recv(ptmp, total, rdtype,
				    root, MCA_COLL_BASE_TAG_SCATTERV,
				    comm, MPI_STATUS_IGNORE));
	    if (OMPI_SUCCESS != err) {
		return err;
	    }
	    /* set the local displacement i.e. no displacements here */
	    displace = (ptrdiff_t *)malloc(sizeof(ptrdiff_t) * size_local);
	    displace[0] = 0;
	    for (i = 1; i < size_local; i++) {
		displace[i] = displace[i-1] + counts[i-1];
	    }
	}
	/* perform the scatterv locally */
    OMPI_COUNT_ARRAY_INIT(&counts_arg, counts);
    OMPI_DISP_ARRAY_INIT(&displace_arg, displace);
	err = comm->c_local_comm->c_coll->coll_scatterv(ptmp, &counts_arg, &displace_arg,
						       rdtype, rbuf, rcount,
						       rdtype, 0, comm->c_local_comm,
                                                       comm->c_local_comm->c_coll->coll_scatterv_module);
	if (OMPI_SUCCESS != err) {
	    return err;
	}

	if (NULL != ptmp_free) {
	    free(ptmp_free);
	}
	if (NULL != displace) {
	    free(displace);
	}
	if (NULL != counts) {
	    free(counts);
	}

    } else {
	err = MCA_PML_CALL(send(scounts, size, MPI_INT, 0,
				MCA_COLL_BASE_TAG_SCATTERV,
				MCA_PML_BASE_SEND_STANDARD, comm));
	if (OMPI_SUCCESS != err) {
	    return err;
	}

    /* TODO:BIGCOUNT: Remove these temporaries once ompi_datatype is updated for bigcount */
    int *tmp_scounts = malloc(sizeof(int) * size);
    int *tmp_disps = malloc(sizeof(int) * size);
    if (NULL == tmp_scounts || NULL == tmp_disps) {
        return OMPI_ERR_OUT_OF_RESOURCE;
    }
    for (i = 0; i < size; ++i) {
        tmp_scounts[i] = (int) ompi_count_array_get(scounts, i);
        tmp_disps[i] = (int) ompi_disp_array_get(disps, i);
    }
	ompi_datatype_create_indexed(size,tmp_scounts,tmp_disps,sdtype,&ndtype);
	ompi_datatype_commit(&ndtype);
    free(tmp_scounts);
    free(tmp_disps);

	err = MCA_PML_CALL(send(sbuf, 1, ndtype, 0,
				MCA_COLL_BASE_TAG_SCATTERV,
				MCA_PML_BASE_SEND_STANDARD, comm));
	if (OMPI_SUCCESS != err) {
	    return err;
	}
	ompi_datatype_destroy(&ndtype);

    }

    /* All done */
    return err;
}
