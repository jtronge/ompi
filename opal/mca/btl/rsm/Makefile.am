if OPAL_ENABLE_RUST_MODULES

MOSTLYCLEANFILES = Cargo.lock opal.rs
BINDGEN_FLAGS = --blocklist-item IPPORT_RESERVED

opal.rs: opal.h
	$(BINDGEN) $(BINDGEN_FLAGS) $< -- $(DEFAULT_INCLUDES) $(INCLUDES) $(CPPFLAGS) > $@
target/debug/librsm.a: Cargo.toml lib.rs opal.rs shared.rs endpoint.rs module.rs modex.rs proc_info.rs local_data.rs
	$(CARGO) build

# Configure for DSO or static builds
if MCA_BUILD_opal_btl_rsm_DSO
component_noinst =
component_install = mca_btl_rsm.la
else
component_noinst = libmca_btl_rsm.la
component_install =
endif

mcacomponentdir = $(opallibdir)
mcacomponent_LTLIBRARIES = $(component_install) target/debug/librsm.a

mca_btl_rsm_la_SOURCES = component.c
mca_btl_rsm_la_LDFLAGS = -module -avoid-version
mca_btl_rsm_la_LIBADD = $(top_builddir)/opal/lib@OPAL_LIB_NAME@.la -L$(shell pwd)/target/debug -lrsm
mca_btl_rsm_la_DEPENDENCIES = target/debug/librsm.a

noinst_LTLIBRARIES = $(component_noinst)
libmca_btl_rsm_la_SOURCES = component.c
libmca_btl_rsm_la_LIBADD = -L$(shell pwd)/target/debug -lrsm
libmca_btl_rsm_la_LDFLAGS = -module -avoid-version
libmca_btl_rsm_la_DEPENDENCIES = target/debug/librsm.a

clean-local:
	-rm -rf target

endif # OPAL_ENABLE_RUST_MODULES
