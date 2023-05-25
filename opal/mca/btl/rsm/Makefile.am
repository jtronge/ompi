if OPAL_ENABLE_RUST_MODULES

MOSTLYCLEANFILES = Cargo.lock opal.rs
BINDGEN_FLAGS = --blocklist-item IPPORT_RESERVED

opal.rs: opal.h
	$(BINDGEN) $(BINDGEN_FLAGS) $< -- $(DEFAULT_INCLUDES) $(INCLUDES) $(CPPFLAGS) > $@
target/release/librsm.a: Cargo.toml lib.rs opal.rs
	$(CARGO) build --release

mcacomponentdir = $(opallibdir)

noinst_LTLIBRARIES = libmca_btl_rsm.la
libmca_btl_rsm_la_SOURCES = component.c
libmca_btl_rsm_la_LIBS = $(btl_ofi_LIBS)
libmca_btl_rsm_la_LIBADD = target/release/librsm.a

clean-local:
	-rm -rf target

endif # OPAL_ENABLE_RUST_MODULES
