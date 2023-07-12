if OPAL_ENABLE_RUST_MODULES

MOSTLYCLEANFILES = Cargo.lock opal.rs
BINDGEN_FLAGS = --blocklist-item IPPORT_RESERVED
# TODO: Need to determine how to switch between --debug/--release versions
CARGO_BUILD_ARGS = --release
TARGET_DIR = target/release
# CARGO_BUILD_ARGS = --debug
# TARGET_DIR = target/debug

opal.rs: opal.h
	$(BINDGEN) $(BINDGEN_FLAGS) $< -- $(DEFAULT_INCLUDES) $(INCLUDES) $(CPPFLAGS) > $@
$(TARGET_DIR)/librsm.a: Cargo.toml lib.rs opal.rs shared.rs fifo.rs block_store.rs endpoint.rs module.rs modex.rs proc_info.rs local_data.rs
	$(CARGO) build $(CARGO_BUILD_ARGS)

# Configure for DSO or static builds
if MCA_BUILD_opal_btl_rsm_DSO
component_noinst =
component_install = mca_btl_rsm.la
else
component_noinst = libmca_btl_rsm.la
component_install =
endif

mcacomponentdir = $(opallibdir)
mcacomponent_LTLIBRARIES = $(component_install) $(TARGET_DIR)/librsm.a

mca_btl_rsm_la_SOURCES = component.c
mca_btl_rsm_la_LDFLAGS = -module -avoid-version
# TODO: What should be used instead of `shell pwd` to get to the target dir
mca_btl_rsm_la_LIBADD = $(top_builddir)/opal/lib@OPAL_LIB_NAME@.la -L$(shell pwd)/$(TARGET_DIR) -lrsm
mca_btl_rsm_la_DEPENDENCIES = $(TARGET_DIR)/librsm.a

noinst_LTLIBRARIES = $(component_noinst)
libmca_btl_rsm_la_SOURCES = component.c
libmca_btl_rsm_la_LIBADD = -L$(shell pwd)/$(TARGET_DIR) -lrsm
libmca_btl_rsm_la_LDFLAGS = -module -avoid-version
libmca_btl_rsm_la_DEPENDENCIES = $(TARGET_DIR)/librsm.a

clean-local:
	-rm -rf target

endif # OPAL_ENABLE_RUST_MODULES
