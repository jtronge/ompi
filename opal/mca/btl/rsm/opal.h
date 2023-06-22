#include "opal_config.h"
#include "opal/mca/shmem/base/base.h"
#include "opal/mca/btl/base/base.h"
#include "opal/mca/btl/base/btl_base_error.h"
#include "opal/mca/btl/btl.h"

/* *_rs wrapper functions */
int opal_modex_recv_value_rs(const char *key,
                             const opal_process_name_t *proc_name,
                             void *data, uint32_t data_type);
int opal_modex_recv_string_rs(const char *key,
                              const opal_process_name_t *proc_name,
                              void *data);
int opal_modex_send_string_rs(uint32_t scope, const char *key, void *data, size_t size);
int opal_proc_on_local_node_rs(opal_hwloc_locality_t proc_flags);
void opal_convertor_get_current_pointer_rs(const opal_convertor_t *pConv, void **position);
int opal_convertor_need_buffers_rs(const opal_convertor_t *pConv);

struct mca_btl_rsm_t {
    /* Actual module instance */
    mca_btl_base_module_t parent;
    /* Internal data used by the module */
    void *internal;
};
