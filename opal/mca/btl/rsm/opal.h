#include "opal_config.h"
#include "opal/mca/shmem/base/base.h"
#include "opal/mca/btl/base/base.h"
#include "opal/mca/btl/base/btl_base_error.h"
#include "opal/mca/btl/btl.h"

/* *_rs wrapper functions */
int opal_modex_recv_value_rs(const char *key,
                             const opal_process_name_t *proc_name,
                             void *data, uint32_t data_type);
int opal_proc_on_local_node_rs(opal_hwloc_locality_t proc_flags);
