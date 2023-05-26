#include "opal.h"

/* Component functions implemented in Rust */
int mca_btl_rsm_component_progress(void);
int mca_btl_rsm_component_open(void);
int mca_btl_rsm_component_close(void);
int mca_btl_rsm_component_register_params(void);
mca_btl_base_module_t **mca_btl_rsm_component_init(int *num_btls,
                                                   bool enable_progress_threads,
                                                   bool enable_mpi_threads);

/**
 * Rust Shared Memory (RSM) component instance.
 */
mca_btl_base_component_3_0_0_t mca_btl_rsm_component = {
    .btl_version = {
        MCA_BTL_DEFAULT_VERSION("rsm"),
        .mca_open_component = mca_btl_rsm_component_open,
        .mca_close_component = mca_btl_rsm_component_close,
        .mca_register_component_params = mca_btl_rsm_component_register_params,
    },
    /* .btl_data = {}, */
    .btl_init = mca_btl_rsm_component_init,
    .btl_progress = mca_btl_rsm_component_progress,
};

/* Module functions implemented in Rust */
int mca_btl_rsm_add_procs(struct mca_btl_base_module_t *btl, size_t nprocs,
                          struct opal_proc_t **procs,
                          struct mca_btl_base_endpoint_t **peers,
                          opal_bitmap_t *reachability);
int mca_btl_rsm_del_procs(struct mca_btl_base_module_t *btl, size_t nprocs,
                          struct opal_proc_t **procs,
                          struct mca_btl_base_endpoint_t **peers);
int mca_btl_rsm_finalize(struct mca_btl_base_module_t *btl);
mca_btl_base_descriptor_t *mca_btl_rsm_alloc(struct mca_btl_base_module_t *btl,
                                             struct mca_btl_base_endpoint_t *endpoint,
                                             uint8_t order, size_t size,
                                             uint32_t flags);
int mca_btl_rsm_free(struct mca_btl_base_module_t *btl,
                     mca_btl_base_descriptor_t *des);
struct mca_btl_base_descriptor_t *mca_btl_rsm_prepare_src(struct mca_btl_base_module_t *btl,
                                                          struct mca_btl_base_endpoint_t *endpoint,
                                                          struct opal_convertor_t *convertor,
                                                          uint8_t order, size_t reserve, size_t *size,
                                                          uint32_t flags);
int mca_btl_rsm_send(struct mca_btl_base_module_t *btl, struct mca_btl_base_endpoint_t *endpoint,
                     struct mca_btl_base_descriptor_t *descriptor, mca_btl_base_tag_t tag);
int mca_btl_rsm_sendi(struct mca_btl_base_module_t *btl, struct mca_btl_base_endpoint_t *endpoint,
                      struct opal_convertor_t *convertor, void *header, size_t header_size,
                      size_t payload_size, uint8_t order, uint32_t flags, mca_btl_base_tag_t tag,
                      mca_btl_base_descriptor_t **descriptor);
int mca_btl_rsm_register_error(struct mca_btl_base_module_t *btl,
                               mca_btl_base_module_error_cb_fn_t cbfunc);

/**
 * Rust Shared Memory (RSM) module instance.
 */
mca_btl_base_module_t mca_btl_rsm = {
    &mca_btl_rsm_component,
    .btl_add_procs = mca_btl_rsm_add_procs,
    .btl_del_procs = mca_btl_rsm_del_procs,
    .btl_finalize = mca_btl_rsm_finalize,
    .btl_alloc = mca_btl_rsm_alloc,
    .btl_free = mca_btl_rsm_free,
    .btl_prepare_src = mca_btl_rsm_prepare_src,
    .btl_send = mca_btl_rsm_send,
    .btl_sendi = mca_btl_rsm_sendi,
    .btl_dump = mca_btl_base_dump,
    .btl_register_error = mca_btl_rsm_register_error,
};

/**
 * *_rs wrapping functions for macros and other C-idioms.
 */
int opal_modex_recv_value_rs(const char *key,
                             const opal_process_name_t *proc_name,
                             void *data, uint32_t data_type)
{
    int rc;
    OPAL_MODEX_RECV_VALUE(rc, key, proc_name, data, data_type);
    return rc;
}

int opal_proc_on_local_node_rs(opal_hwloc_locality_t proc_flags)
{
    return OPAL_PROC_ON_LOCAL_NODE(proc_flags);
}
