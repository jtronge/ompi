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

#include <stdio.h>
#include "ompi_config.h"
#include "ompi/datatype/ompi_datatype.h"
#include "ompi/datatype/ompi_datatype_internal.h"
#include "mpi.h"

/*
 * Internal enums/structs for a simpler type signature representation.
 */
enum typesig_type {
    TYPESIG_TYPE_STRUCT,
    TYPESIG_TYPE_VECTOR,
    TYPESIG_TYPE_PREDEFINED,
};

struct typesig {
    enum typesig_type type;
    union {
        /* Vector-like type signature */
        struct {
            int count;
            struct typesig *sig;
        } vec;
        /* Struct-like type signature */
        struct {
            int count;
            int *blocklens;
            struct typesig **sigs;
        } st;
        /* Predefined type */
        unsigned char pre;
    } d;
};

static struct typesig *typesig_vector_new(int count, struct typesig *sig);
static struct typesig *typesig_struct_new(int count, int *blocklens, struct typesig **sigs);
static struct typesig *typesig_predefined_new(unsigned char pre);
static void typesig_free(struct typesig *sig);
static struct typesig *typesig_dup(struct typesig *sig);
static uint64_t typesig_hash(struct typesig *sig);

static unsigned char id2sigval(int id);
static void fnv1_hash_init(uint64_t *hash);
static void fnv1_hash_update(uint64_t *hash, unsigned char *data, int n);

/*
 * Compute the type signature and hash for a "vector"-like datatype (where all
 * elements are of the same type).
 */
void ompi_datatype_build_typesig_vector_like(ompi_datatype_t *type,
                                             const ompi_datatype_t *inner, int count)
{
    struct typesig *inner_sig;
    struct typesig *sig;

    /* Build up the type signature */
    if (ompi_datatype_is_predefined(inner)) {
        inner_sig = typesig_predefined_new(id2sigval(inner->id));
    } else {
        inner_sig = typesig_dup(inner->sig);
    }
    if (inner_sig == NULL) {
        /* TODO: Is this a fatal error? */
        return;
    }
    sig = typesig_vector_new(count, inner_sig);
    if (sig == NULL) {
        typesig_free(inner_sig);
        return;
    }
    type->sig = sig;
    /* Now calculate the hash */
    if (ompi_datatype_is_predefined(inner)) {
        type->unit_hash = ompi_datatype_predefined_hashes[inner->id];
    } else {
        type->unit_hash = inner->unit_hash;
    }
    type->full_hash = typesig_hash(sig);
}

/*
 * Compute the type signature and hash for a struct-like datatype.
 */
void ompi_datatype_build_typesig_struct(ompi_datatype_t *type, int count,
                                        const int *blocklens,
                                        ompi_datatype_t *const *inner_types)
{
    int *blens = NULL;
    struct typesig **inner_sigs = NULL;
    struct typesig *sig = NULL;
    int i;

    if (count == 1) {
        ompi_datatype_build_typesig_vector_like(type, inner_types[0], count);
        return;
    }
    blens = malloc(count * sizeof(*blens));
    if (NULL == blens) {
        return;
    }
    inner_sigs = malloc(count * sizeof(*inner_sigs));
    if (NULL == inner_sigs) {
        free(blens);
        return;
    }
    for (i = 0; i < count; ++i) {
        blens[i] = blocklens[i];
        if (ompi_datatype_is_predefined(inner_types[i])) {
            inner_sigs[i] = typesig_predefined_new(id2sigval(inner_types[i]->id));
        } else {
            inner_sigs[i] = typesig_dup(inner_types[i]->sig);
        }
        if (NULL == inner_sigs[i]) {
            free(blens);
            for (i = i - 1; i >= 0; i--) {
                typesig_free(inner_sigs[i]);
            }
            return;
        }
    }
    sig = typesig_struct_new(count, blens, inner_sigs);
    if (NULL == sig) {
        free(blens);
        for (i = 0; i < count; ++i) {
            typesig_free(inner_sigs[i]);
        }
        free(inner_sigs);
        return;
    }
    type->sig = sig;
    /* Calculate the hash */
    type->full_hash = typesig_hash(sig);
    type->unit_hash = type->full_hash;
}

/*
 * Build the type signature and hash for a multi-dimensional array-like
 * structure.
 */
void ompi_datatype_build_typesig_multi_dim_array(ompi_datatype_t *newtype,
                                                 const ompi_datatype_t *oldtype,
                                                 int ndims,
                                                 int const *size_array)
{
    int i, count;

    count = size_array[0];
    for (i = 1; i < ndims; i++) {
        count *= size_array[i];
    }
    ompi_datatype_build_typesig_vector_like(newtype, oldtype, count);
}

/*
 * Hash a predefined type (assuming the number of predefined types < 256).
 */
uint64_t ompi_datatype_hash_predefined(int id)
{
    uint64_t hash;
    unsigned char b = id2sigval(id);

    fnv1_hash_init(&hash);
    fnv1_hash_update(&hash, &b, 1);
    return hash;
}

/*
 * Free any internal data dealing with the type signature.
 */
void ompi_datatype_typesig_free(void *sig)
{
    if (NULL != sig) {
        typesig_free(sig);
    }
}

/*
 * Duplicate the internal type signature. Returns non-zero on malloc failure.
 */
int ompi_datatype_typesig_duplicate(const ompi_datatype_t *old,
                                    ompi_datatype_t *type)
{
    struct typesig *sig;

    if (ompi_datatype_is_predefined(old)) {
        sig = typesig_predefined_new(id2sigval(old->id));
    } else {
        sig = typesig_dup(old->sig);
    }
    if (sig == NULL) {
        return 1;
    }
    type->sig = sig;
    type->full_hash = old->full_hash;
    type->unit_hash = old->unit_hash;
    return 0;
}

static struct typesig *typesig_vector_new(int count, struct typesig *sig)
{
    struct typesig *new;

    new = malloc(sizeof(*sig));
    if (NULL == new) {
        return NULL;
    }
    new->type = TYPESIG_TYPE_VECTOR;
    new->d.vec.count = count;
    new->d.vec.sig = sig;
    return new;
}

static struct typesig *typesig_struct_new(int count, int *blocklens,
                                          struct typesig **sigs)
{
    struct typesig *new;

    new = malloc(sizeof(*new));
    if (NULL == new) {
        return NULL;
    }
    new->type = TYPESIG_TYPE_STRUCT;
    new->d.st.count = count;
    new->d.st.blocklens = blocklens;
    new->d.st.sigs = sigs;
    return new;
}

static struct typesig *typesig_predefined_new(unsigned char pre)
{
    struct typesig *new;

    new = malloc(sizeof(*new));
    if (NULL == new) {
        return NULL;
    }
    new->type = TYPESIG_TYPE_PREDEFINED;
    new->d.pre = pre;
    return new;
}

static void typesig_free(struct typesig *sig)
{
    int i;
    switch (sig->type) {
    case TYPESIG_TYPE_VECTOR:
        typesig_free(sig->d.vec.sig);
        break;
    case TYPESIG_TYPE_STRUCT:
        free(sig->d.st.blocklens);
        for (i = 0; i < sig->d.st.count; i++) {
            typesig_free(sig->d.st.sigs[i]);
        }
        free(sig->d.st.sigs);
        break;
    case TYPESIG_TYPE_PREDEFINED:
        break;
    }
    free(sig);
}

static struct typesig *typesig_dup(struct typesig *sig)
{
    int *blocklens;
    struct typesig **sigs;
    struct typesig *tmp;
    int i;

    switch (sig->type) {
    case TYPESIG_TYPE_VECTOR:
        tmp = typesig_dup(sig->d.vec.sig);
        if (NULL == tmp) {
            return NULL;
        }
        return typesig_vector_new(sig->d.vec.count, tmp);
    case TYPESIG_TYPE_STRUCT:
        blocklens = malloc(sig->d.st.count * sizeof(*blocklens));
        if (NULL == blocklens) {
            return NULL;
        }
        sigs = malloc(sig->d.st.count * sizeof(*sigs));
        if (NULL == sigs) {
            free(blocklens);
            return NULL;
        }
        for (i = 0; i < sig->d.st.count; i++) {
            blocklens[i] = sig->d.st.blocklens[i];
            sigs[i] = typesig_dup(sig->d.st.sigs[i]);
            if (sigs[i] == NULL) {
                for (i = i - 1; i >= 0; i--) {
                    typesig_free(sigs[i]);
                }
                free(blocklens);
                free(sigs);
                return NULL;
            }
        }
        return typesig_struct_new(sig->d.st.count, blocklens, sigs);
    case TYPESIG_TYPE_PREDEFINED:
        return typesig_predefined_new(sig->d.pre);
    }

    /* Shouldn't be reachable */
    return NULL;
}

static void typesig_hash_update(struct typesig *sig, uint64_t *hash);

/* Compute a hash for the type signature */
static uint64_t typesig_hash(struct typesig *sig)
{
    uint64_t hash;

    fnv1_hash_init(&hash);
    typesig_hash_update(sig, &hash);
    return hash;
}

static void typesig_hash_update(struct typesig *sig, uint64_t *hash)
{
    int i, j;

    switch (sig->type) {
    case TYPESIG_TYPE_VECTOR:
        for (i = 0; i < sig->d.vec.count; i++) {
            typesig_hash_update(sig->d.vec.sig, hash);
        }
        break;
    case TYPESIG_TYPE_STRUCT:
        for (i = 0; i < sig->d.st.count; i++) {
            for (j = 0; j < sig->d.st.blocklens[i]; j++) {
                typesig_hash_update(sig->d.st.sigs[i], hash);
            }
        }
        break;
    case TYPESIG_TYPE_PREDEFINED:
        fnv1_hash_update(hash, &sig->d.pre, 1);
        break;
    }
}

/* Convert an ID for a predefined type into a signature value */
static unsigned char id2sigval(int id)
{
    return id + 1;
}

/*
 * Non-cryptographic Fowler-Noll-Vo hash. See
 * https://en.wikipedia.org/wiki/Fowler%E2%80%93Noll%E2%80%93Vo_hash_function
 * for more info.
 */

#define FNV_OFFSET_BASIS    0xcbf29ce484222325
#define FNV_PRIME           0x100000001b3

/* Initialize the hash */
static void fnv1_hash_init(uint64_t *hash)
{
    *hash = FNV_OFFSET_BASIS;
}

/* Update the hash (note that you can simply use the value stored in *hash once done) */
static void fnv1_hash_update(uint64_t *hash, unsigned char *data, int n)
{
    int i;

    for (i = 0; i < n; ++i) {
        *hash *= FNV_PRIME;
        *hash ^= data[i];
    }
}
