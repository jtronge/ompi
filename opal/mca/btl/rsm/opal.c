#include "opal.h"

int mca_btl_rsm_component_progress(void);
int mca_btl_rsm_component_open(void);
int mca_btl_rsm_component_close(void);
int mca_btl_rsm_component_register(void);
mca_btl_base_module_t **mca_btl_rsm_component_init(int *num_btls,
                                                   bool enable_progress_threads,
                                                   bool enable_mpi_threads);
