/*
 * Copyright (c) 2024   Triad National Security, LLC. All rights reserved.
 * $COPYRIGHT$
 *
 * Additional copyrights may follow
 *
 * $HEADER$
 */
#ifndef OMPI_UTIL_COUNT_DISP_ARRAY_H
#define OMPI_UTIL_COUNT_DISP_ARRAY_H

#include <stdlib.h>
#include <stddef.h>
#include <stdint.h>

enum ompi_count_array_type {
    OMPI_COUNT_ARRAY_TYPE_INT,
    OMPI_COUNT_ARRAY_TYPE_SIZE_T,
};

typedef struct ompi_count_array {
    enum ompi_count_array_type type;
    union {
        const int *int_array;
        const size_t *size_t_array;
    } data;
} ompi_count_array;

/* Initialize a bigcount variant of the count array */
static inline void ompi_count_array_init_c(ompi_count_array *array, const size_t *data)
{
    array->type = OMPI_COUNT_ARRAY_TYPE_SIZE_T;
    array->data.size_t_array = data;
}

/* Get a count in the array at index i */
static inline size_t ompi_count_array_get(ompi_count_array *array, size_t i)
{
    switch (array->type) {
    case OMPI_COUNT_ARRAY_TYPE_INT:
        return array->data.int_array[i];
    case OMPI_COUNT_ARRAY_TYPE_SIZE_T:
        return array->data.size_t_array[i];
    default:
        /* This shouldn't be possible */
        return 0;
    }
}

enum ompi_disp_array_type {
    OMPI_DISP_ARRAY_TYPE_INT,
    OMPI_DISP_ARRAY_TYPE_PTRDIFF_T,
};

typedef struct ompi_disp_array {
    enum ompi_count_array_type type;
    union {
        const int *int_array;
        const ptrdiff_t *ptrdiff_t_array;
    } data;
} ompi_disp_array;

/* Initialize a bigcount variant of the disp array */
static inline void ompi_disp_array_init_c(ompi_disp_array *array, const ptrdiff_t *data)
{
    array->type = OMPI_DISP_ARRAY_TYPE_PTRDIFF_T;
    array->data.ptrdiff_t_array = data;
}

/* Get a displacement in the array at index i */
static inline ptrdiff_t ompi_disp_array_get(ompi_disp_array *array, size_t i)
{
    switch (array->type) {
    case OMPI_DISP_ARRAY_TYPE_INT:
        return array->data.int_array[i];
    case OMPI_DISP_ARRAY_TYPE_PTRDIFF_T:
        return array->data.ptrdiff_t_array[i];
    default:
        /* This shouldn't be possible */
        return 0;
    }
}

#endif
