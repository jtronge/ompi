/* -*- Mode: C; c-basic-offset:4 ; -*- */
/*
 * Copyright (c) 2022      Triad National Security, LLC. All rights
 *                         reserved.
 * $COPYRIGHT$
 *
 * Additional copyrights may follow
 *
 * $HEADER$
 */

#include "ompi_config.h"
#include "ompi/datatype/ompi_datatype.h"
#include "opal/runtime/opal.h"
#include "ddt_lib.h"

int main(void)
{
    int ret = 0;
    ompi_datatype_t *vec1, *vec2, *vec3, *hvec, *st, *subarray, *vec4;
    int size_array[] = {3, 3, 3, 3};
    int subsize_array[] = {3, 3, 3, 3};
    int start_array[] = {0, 0, 0, 0};
    uint64_t tmp_hash;

    opal_init(NULL, NULL);
    ompi_datatype_init();

#define CHECK_ERR(expr) do { \
    if (0 != (expr)) {       \
        ret = 1;            \
        goto cleanup;       \
    }                       \
} while (0)

#define ASSERT(expr) do {                           \
    if (!(expr)) {                                  \
        fprintf(stderr, "ERROR: " #expr " is FALSE\n"); \
        ret = 1;                                    \
        goto cleanup;                               \
    }                                               \
} while (0)

    printf("---> Check that vector and hvector have identical hashes\n");
    CHECK_ERR(ompi_datatype_create_vector(4, 1, 1, &ompi_mpi_int.dt, &vec1));
    CHECK_ERR(ompi_datatype_create_hvector(4, 1, 1, &ompi_mpi_int.dt, &hvec));
    printf("-------> vec1->full_hash: %lx\n", vec1->full_hash);
    printf("-------> hvec->full_hash: %lx\n", hvec->full_hash);
    printf("-------> vec1->unit_hash: %lx\n", vec1->unit_hash);
    printf("-------> hvec->unit_hash: %lx\n", hvec->unit_hash);
    ASSERT(vec1->full_hash != 0);
    ASSERT(hvec->full_hash != 0);
    ASSERT(vec1->unit_hash != 0);
    ASSERT(hvec->unit_hash != 0);
    ASSERT(vec1->full_hash == hvec->full_hash);
    ASSERT(vec1->unit_hash == hvec->unit_hash);
    ompi_datatype_destroy(&vec1);
    ompi_datatype_destroy(&hvec);

    printf("---> Check that a vector and a single element have the same unit_hash\n");
    CHECK_ERR(ompi_datatype_create_vector(3, 1, 1, &ompi_mpi_double.dt, &vec2));
    CHECK_ERR(ompi_datatype_create_vector(1, 1, 1, &ompi_mpi_double.dt, &vec3));
    printf("-------> vec2->full_hash: %lx\n", vec2->full_hash);
    printf("-------> vec2->unit_hash: %lx\n", vec2->unit_hash);
    printf("-------> vec3->full_hash: %lx\n", vec3->full_hash);
    printf("-------> vec3->unit_hash: %lx\n", vec3->unit_hash);
    ASSERT(vec2->unit_hash == vec3->unit_hash);
    ASSERT(vec2->unit_hash == vec3->unit_hash);
    ASSERT(vec2->full_hash != vec3->full_hash);
    assert(vec3->full_hash == vec3->unit_hash);
    ompi_datatype_destroy(&vec2);
    ompi_datatype_destroy(&vec3);

    printf("---> Check the hash of a struct datatype\n");
    st = create_strange_dt();
    printf("-------> st->full_hash: %lx\n", st->full_hash);
    printf("-------> st->unit_hash: %lx\n", st->unit_hash);
    ASSERT(st->unit_hash != 0);
    ASSERT(st->full_hash != 0);
    ompi_datatype_destroy(&st);

    printf("---> Compare the hash of a multi-dimensional subarray and a vector\n");
    CHECK_ERR(ompi_datatype_create_subarray(4, size_array, subsize_array,
                                            start_array, MPI_ORDER_C,
                                            &ompi_mpi_float.dt, &subarray));
    CHECK_ERR(ompi_datatype_create_vector(3 * 3 * 3 * 3, 1, 1,
                                          &ompi_mpi_float.dt, &vec4));
    printf("-------> subarray->full_hash: %lx\n", subarray->full_hash);
    printf("-------> subarray->unit_hash: %lx\n", subarray->unit_hash);
    printf("-------> vec4->full_hash: %lx\n", vec4->full_hash);
    printf("-------> vec4->unit_hash: %lx\n", vec4->unit_hash);
    ASSERT(subarray->full_hash != 0);
    ASSERT(subarray->unit_hash != 0);
    ASSERT(subarray->full_hash == vec4->full_hash);
    ASSERT(subarray->unit_hash == vec4->unit_hash);
    ompi_datatype_destroy(&subarray);
    ompi_datatype_destroy(&vec4);

    printf("---> Trying ompi_datatype_get_typesig_hash() on predefined types\n");
    tmp_hash = ompi_datatype_get_typesig_hash(&ompi_mpi_double.dt);
    printf("-------> hash(MPI_DOUBLE) = %lx\n", tmp_hash);
    assert(tmp_hash != 0);
    tmp_hash = ompi_datatype_get_typesig_hash(&ompi_mpi_float.dt);
    printf("-------> hash(MPI_FLOAT) = %lx\n", tmp_hash);
    assert(tmp_hash != 0);
    tmp_hash = ompi_datatype_get_typesig_hash(&ompi_mpi_int.dt);
    printf("-------> hash(MPI_INT) = %lx\n", tmp_hash);
    assert(tmp_hash != 0);

cleanup:
    opal_finalize_util();
    return ret;
}
