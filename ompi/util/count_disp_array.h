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

/* Initialize an int variant of the count array */
static inline void ompi_count_array_init(ompi_count_array *array, const int *data)
{
    array->type = OMPI_COUNT_ARRAY_TYPE_INT;
    array->data.int_array = data;
}

/* Initialize a bigcount variant of the count array */
static inline void ompi_count_array_init_c(ompi_count_array *array, const size_t *data)
{
    array->type = OMPI_COUNT_ARRAY_TYPE_SIZE_T;
    array->data.size_t_array = data;
}

#define OMPI_COUNT_ARRAY_INIT(array, data) _Generic((data), \
                                                    int *: ompi_count_array_init, \
                                                    const int *: ompi_count_array_init, \
                                                    size_t *: ompi_count_array_init_c, \
                                                    const size_t *: ompi_count_array_init_c)(array, data)

/* Return if the internal type is 64-bit or not */
static inline bool ompi_count_array_is_64bit(ompi_count_array *array)
{
    return array->type == OMPI_COUNT_ARRAY_TYPE_SIZE_T;
}

static inline const void *ompi_count_array_ptr(ompi_count_array *array)
{
    if (OPAL_LIKELY(array->type == OMPI_COUNT_ARRAY_TYPE_INT))
        return array->data.int_array;
    return array->data.size_t_array;
}

/* Get a count in the array at index i */
static inline size_t ompi_count_array_get(ompi_count_array *array, size_t i)
{
    if (OPAL_LIKELY(array->type == OMPI_COUNT_ARRAY_TYPE_INT))
        return array->data.int_array[i];
    return array->data.size_t_array[i];
}

enum ompi_disp_array_type {
    OMPI_DISP_ARRAY_TYPE_INT,
    OMPI_DISP_ARRAY_TYPE_PTRDIFF_T,
};

typedef struct ompi_disp_array {
    enum ompi_disp_array_type type;
    union {
        const int *int_array;
        const ptrdiff_t *ptrdiff_t_array;
    } data;
} ompi_disp_array;

/* Initialize an int variant of the disp array */
static inline void ompi_disp_array_init(ompi_disp_array *array, const int *data)
{
    array->type = OMPI_DISP_ARRAY_TYPE_INT;
    array->data.int_array = data;
}

/* Initialize a bigcount variant of the disp array */
static inline void ompi_disp_array_init_c(ompi_disp_array *array, const ptrdiff_t *data)
{
    array->type = OMPI_DISP_ARRAY_TYPE_PTRDIFF_T;
    array->data.ptrdiff_t_array = data;
}

#define OMPI_DISP_ARRAY_INIT(array, data) _Generic((data), \
                                                   int *: ompi_disp_array_init, \
                                                   const int *: ompi_disp_array_init, \
                                                   ptrdiff_t *: ompi_disp_array_init_c, \
                                                   const ptrdiff_t *: ompi_disp_array_init_c)(array, data)

/* Return if the internal type is 64-bit or not */
static inline bool ompi_disp_array_is_64bit(ompi_disp_array *array)
{
    return array->type == OMPI_DISP_ARRAY_TYPE_PTRDIFF_T;
}

/* Get a displacement in the array at index i */
static inline ptrdiff_t ompi_disp_array_get(ompi_disp_array *array, size_t i)
{
    if (OPAL_LIKELY(array->type == OMPI_DISP_ARRAY_TYPE_INT))
        return array->data.int_array[i];
    return array->data.ptrdiff_t_array[i];
}

static inline const void *ompi_disp_array_ptr(ompi_disp_array *array)
{
    if (OPAL_LIKELY(array->type == OMPI_DISP_ARRAY_TYPE_INT))
        return array->data.int_array;
    return array->data.ptrdiff_t_array;
}

#endif
