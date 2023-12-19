# -*- makefile.am -*-
#
# Copyright (c) 2006-2019 Cisco Systems, Inc.  All rights reserved.
# Copyright (c) 2012-2013 The University of Tennessee and The University
#                         of Tennessee Research Foundation.  All rights
#                         reserved.
# Copyright (c) 2012-2013 Inria.  All rights reserved.
# Copyright (c) 2013-2018 Los Alamos National Security, LLC. All rights
#                         reserved.
# Copyright (c) 2015-2021 Research Organization for Information Science
#                         and Technology (RIST).  All rights reserved.
# Copyright (c) 2016      IBM Corporation.  All rights reserved.
# Copyright (c) 2017-2018 FUJITSU LIMITED.  All rights reserved.
# Copyright (c) 2019-2022 Triad National Security, LLC. All rights
#                         reserved.
# Copyright (c) 2020      Sandia National Laboratories. All rights reserved.
# Copyright (c) 2022      IBM Corporation.  All rights reserved.
#
# $COPYRIGHT$
#
# Additional copyrights may follow
#
# $HEADER$
#

include $(top_srcdir)/Makefile.ompi-rules

# Note that Automake's Fortran-buidling rules uses CPPFLAGS and
# AM_CPPFLAGS.  This can cause weirdness (e.g.,
# https://github.com/open-mpi/ompi/issues/7253).  Let's just zero
# those out and rely on AM_FCFLAGS.
CPPFLAGS =
AM_CPPFLAGS =

# This Makefile is only relevant if we're building the "use mpi_f08"
# MPI bindings.
if OMPI_BUILD_FORTRAN_USEMPIF08_BINDINGS

AM_FCFLAGS = -I$(top_srcdir)/ompi/mpi/fortran/use-mpi-f08/mod \
             -I$(top_builddir)/ompi/include \
             -I$(top_srcdir)/ompi/include \
             $(OMPI_FC_MODULE_FLAG)$(top_builddir)/ompi/mpi/fortran/use-mpi \
             $(OMPI_FC_MODULE_FLAG)$(top_builddir)/ompi/$(OMPI_FORTRAN_USEMPI_DIR) \
             $(OMPI_FC_MODULE_FLAG)mod \
             $(OMPI_FC_MODULE_FLAG)bindings \
             -I$(top_srcdir) -I$(top_builddir) $(FCFLAGS_f90)

MOSTLYCLEANFILES = *.mod

CLEANFILES += *.i90

lib_LTLIBRARIES = lib@OMPI_LIBMPI_NAME@_usempif08.la
noinst_LTLIBRARIES = lib@OMPI_LIBMPI_NAME@_usempif08_pmpi.la

module_sentinel_files = \
        mod/libforce_usempif08_internal_modules_to_be_built.la \
        bindings/libforce_usempif08_internal_bindings_to_be_built.la

mpi-f08.lo: $(module_sentinel_files)
mpi-f08.lo: mpi-f08.F90
mpi-f08.lo: sizeof_f08.h

#
# *sizeof_f08.* are generated based on some results from
# configure tests.
#

sizeof_pl=$(top_srcdir)/ompi/mpi/fortran/base/gen-mpi-sizeof.pl

sizeof_f08.h: $(top_builddir)/config.status
sizeof_f08.h: $(sizeof_pl)
sizeof_f08.h:
	$(OMPI_V_GEN) $(sizeof_pl) \
	    --header=$@ --ierror=optional \
	    --maxrank=$(OMPI_FORTRAN_MAX_ARRAY_RANK) \
	    --generate=$(OMPI_FORTRAN_BUILD_SIZEOF) \
	    --real2=$(OMPI_HAVE_FORTRAN_REAL2) \
	    --iso_real16=$(OMPI_FORTRAN_HAVE_ISO_FORTRAN_ENV_REAL16) \
	    --real16=$(OMPI_HAVE_FORTRAN_REAL16) \
	    --complex4=$(OMPI_HAVE_FORTRAN_COMPLEX4) \
	    --complex32=$(OMPI_HAVE_FORTRAN_COMPLEX32) \
	    --mpi_version=$(MPI_VERSION) \
	    --request_deprecate=$(OMPI_FORTRAN_HAVE_ATTR_DEPRECATED)

sizeof_f08.f90: $(top_builddir)/config.status
sizeof_f08.f90: $(sizeof_pl)
sizeof_f08.f90:
	$(OMPI_V_GEN) $(sizeof_pl) \
	    --impl=$@ --ierror=optional --mpi \
	    --maxrank=$(OMPI_FORTRAN_MAX_ARRAY_RANK) \
	    --generate=$(OMPI_FORTRAN_BUILD_SIZEOF) \
	    --real2=$(OMPI_HAVE_FORTRAN_REAL2) \
	    --iso_real16=$(OMPI_FORTRAN_HAVE_ISO_FORTRAN_ENV_REAL16) \
	    --real16=$(OMPI_HAVE_FORTRAN_REAL16) \
	    --complex4=$(OMPI_HAVE_FORTRAN_COMPLEX4) \
	    --complex32=$(OMPI_HAVE_FORTRAN_COMPLEX32)

psizeof_f08.f90: $(top_builddir)/config.status
psizeof_f08.f90: $(sizeof_pl)
psizeof_f08.f90:
	$(OMPI_V_GEN) $(sizeof_pl) \
	    --impl=$@ --ierror=optional --pmpi \
	    --maxrank=$(OMPI_FORTRAN_MAX_ARRAY_RANK) \
	    --generate=$(OMPI_FORTRAN_BUILD_SIZEOF) \
	    --real2=$(OMPI_HAVE_FORTRAN_REAL2) \
	    --iso_real16=$(OMPI_FORTRAN_HAVE_ISO_FORTRAN_ENV_REAL16) \
	    --real16=$(OMPI_HAVE_FORTRAN_REAL16) \
	    --complex4=$(OMPI_HAVE_FORTRAN_COMPLEX4) \
	    --complex32=$(OMPI_HAVE_FORTRAN_COMPLEX32)

CLEANFILES += sizeof_f08.h sizeof_f08.f90 psizeof_f08.f90

mpi_api_generated_files = \
        send_f08_generated.F90
mpi_api_files = \
        abort_f08.F90 \
        accumulate_f08.F90 \
        add_error_class_f08.F90 \
        add_error_code_f08.F90 \
        add_error_string_f08.F90 \
        aint_add_f08.F90 \
        aint_diff_f08.F90 \
        allgather_f08.F90 \
        allgather_init_f08.F90 \
        allgatherv_f08.F90 \
        allgatherv_init_f08.F90 \
        alloc_mem_f08.F90 \
        allreduce_f08.F90 \
        allreduce_init_f08.F90 \
        alltoall_f08.F90 \
        alltoall_init_f08.F90 \
        alltoallv_f08.F90 \
        alltoallv_init_f08.F90 \
        alltoallw_f08.F90 \
        alltoallw_init_f08.F90 \
        barrier_f08.F90 \
        barrier_init_f08.F90 \
        bcast_f08.F90 \
        bcast_init_f08.F90 \
        bsend_f08.F90 \
        bsend_init_f08.F90 \
        buffer_attach_f08.F90 \
        buffer_detach_f08.F90 \
        cancel_f08.F90 \
        cart_coords_f08.F90 \
        cart_create_f08.F90 \
        cartdim_get_f08.F90 \
        cart_get_f08.F90 \
        cart_map_f08.F90 \
        cart_rank_f08.F90 \
        cart_shift_f08.F90 \
        cart_sub_f08.F90 \
        close_port_f08.F90 \
        comm_accept_f08.F90 \
        comm_call_errhandler_f08.F90 \
        comm_compare_f08.F90 \
        comm_connect_f08.F90 \
        comm_create_errhandler_f08.F90 \
        comm_create_f08.F90 \
        comm_create_from_group_f08.F90 \
        comm_create_group_f08.F90 \
        comm_create_keyval_f08.F90 \
        comm_delete_attr_f08.F90 \
        comm_disconnect_f08.F90 \
        comm_dup_f08.F90 \
        comm_dup_with_info_f08.F90 \
        comm_idup_f08.F90 \
        comm_idup_with_info_f08.F90 \
        comm_free_f08.F90 \
        comm_free_keyval_f08.F90 \
        comm_get_attr_f08.F90 \
        comm_get_errhandler_f08.F90 \
        comm_get_info_f08.F90 \
        comm_get_name_f08.F90 \
        comm_get_parent_f08.F90 \
        comm_group_f08.F90 \
        comm_join_f08.F90 \
        comm_rank_f08.F90 \
        comm_remote_group_f08.F90 \
        comm_remote_size_f08.F90 \
        comm_set_attr_f08.F90 \
        comm_set_errhandler_f08.F90 \
        comm_set_info_f08.F90 \
        comm_set_name_f08.F90 \
        comm_size_f08.F90 \
        comm_spawn_f08.F90 \
        comm_spawn_multiple_f08.F90 \
        comm_split_f08.F90 \
        comm_split_type_f08.F90 \
        comm_test_inter_f08.F90 \
        compare_and_swap_f08.F90 \
        dist_graph_create_adjacent_f08.F90 \
        dist_graph_create_f08.F90 \
        dist_graph_neighbors_count_f08.F90 \
        dist_graph_neighbors_f08.F90 \
        dims_create_f08.F90 \
        errhandler_free_f08.F90 \
        error_class_f08.F90 \
        error_string_f08.F90 \
        exscan_f08.F90 \
        exscan_init_f08.F90 \
        f_sync_reg_f08.F90 \
        fetch_and_op_f08.F90 \
        file_call_errhandler_f08.F90 \
        file_close_f08.F90 \
        file_create_errhandler_f08.F90 \
        file_delete_f08.F90 \
        file_get_amode_f08.F90 \
        file_get_atomicity_f08.F90 \
        file_get_byte_offset_f08.F90 \
        file_get_errhandler_f08.F90 \
        file_get_group_f08.F90 \
        file_get_info_f08.F90 \
        file_get_position_f08.F90 \
        file_get_position_shared_f08.F90 \
        file_get_size_f08.F90 \
        file_get_type_extent_f08.F90 \
        file_get_view_f08.F90 \
        file_iread_at_f08.F90 \
        file_iread_f08.F90 \
        file_iread_at_all_f08.F90 \
        file_iread_all_f08.F90 \
        file_iread_shared_f08.F90 \
        file_iwrite_at_f08.F90 \
        file_iwrite_f08.F90 \
        file_iwrite_at_all_f08.F90 \
        file_iwrite_all_f08.F90 \
        file_iwrite_shared_f08.F90 \
        file_open_f08.F90 \
        file_preallocate_f08.F90 \
        file_read_all_begin_f08.F90 \
        file_read_all_end_f08.F90 \
        file_read_all_f08.F90 \
        file_read_at_all_begin_f08.F90 \
        file_read_at_all_end_f08.F90 \
        file_read_at_all_f08.F90 \
        file_read_at_f08.F90 \
        file_read_f08.F90 \
        file_read_ordered_begin_f08.F90 \
        file_read_ordered_end_f08.F90 \
        file_read_ordered_f08.F90 \
        file_read_shared_f08.F90 \
        file_seek_f08.F90 \
        file_seek_shared_f08.F90 \
        file_set_atomicity_f08.F90 \
        file_set_errhandler_f08.F90 \
        file_set_info_f08.F90 \
        file_set_size_f08.F90 \
        file_set_view_f08.F90 \
        file_sync_f08.F90 \
        file_write_all_begin_f08.F90 \
        file_write_all_end_f08.F90 \
        file_write_all_f08.F90 \
        file_write_at_all_begin_f08.F90 \
        file_write_at_all_end_f08.F90 \
        file_write_at_all_f08.F90 \
        file_write_at_f08.F90 \
        file_write_f08.F90 \
        file_write_ordered_begin_f08.F90 \
        file_write_ordered_end_f08.F90 \
        file_write_ordered_f08.F90 \
        file_write_shared_f08.F90 \
        finalized_f08.F90 \
        finalize_f08.F90 \
        free_mem_f08.F90 \
        gather_f08.F90 \
        gather_init_f08.F90 \
        gatherv_f08.F90 \
        gatherv_init_f08.F90 \
        get_accumulate_f08.F90 \
        get_address_f08.F90 \
        get_count_f08.F90 \
        get_elements_f08.F90 \
        get_elements_x_f08.F90 \
        get_f08.F90 \
        get_library_version_f08.F90 \
        get_processor_name_f08.F90 \
        get_version_f08.F90 \
        graph_create_f08.F90 \
        graphdims_get_f08.F90 \
        graph_get_f08.F90 \
        graph_map_f08.F90 \
        graph_neighbors_count_f08.F90 \
        graph_neighbors_f08.F90 \
        grequest_complete_f08.F90 \
        grequest_start_f08.F90 \
        group_compare_f08.F90 \
        group_difference_f08.F90 \
        group_excl_f08.F90 \
        group_free_f08.F90 \
        group_from_session_pset_f08.F90 \
        group_incl_f08.F90 \
        group_intersection_f08.F90 \
        group_range_excl_f08.F90 \
        group_range_incl_f08.F90 \
        group_rank_f08.F90 \
        group_size_f08.F90 \
        group_translate_ranks_f08.F90 \
        group_union_f08.F90 \
        iallgather_f08.F90 \
        iallgatherv_f08.F90 \
        iallreduce_f08.F90 \
        ialltoall_f08.F90 \
        ialltoallv_f08.F90 \
        ialltoallw_f08.F90 \
        ibarrier_f08.F90 \
        ibcast_f08.F90 \
        ibsend_f08.F90 \
        iexscan_f08.F90 \
        igather_f08.F90 \
        igatherv_f08.F90 \
        improbe_f08.F90 \
        imrecv_f08.F90 \
        ineighbor_allgather_f08.F90 \
        ineighbor_allgatherv_f08.F90 \
        ineighbor_alltoall_f08.F90 \
        ineighbor_alltoallv_f08.F90 \
        ineighbor_alltoallw_f08.F90 \
        info_create_f08.F90 \
        info_create_env_f08.F90 \
        info_delete_f08.F90 \
        info_dup_f08.F90 \
        info_free_f08.F90 \
        info_get_f08.F90 \
        info_get_nkeys_f08.F90 \
        info_get_nthkey_f08.F90 \
        info_get_valuelen_f08.F90 \
        info_get_string_f08.F90 \
        info_set_f08.F90 \
        init_f08.F90 \
        initialized_f08.F90 \
        init_thread_f08.F90 \
        intercomm_create_f08.F90 \
        intercomm_create_from_groups_f08.F90 \
        intercomm_merge_f08.F90 \
        iprobe_f08.F90 \
        irecv_f08.F90 \
        ireduce_f08.F90 \
        ireduce_scatter_f08.F90 \
        ireduce_scatter_block_f08.F90 \
        irsend_f08.F90 \
        iscan_f08.F90 \
        iscatter_f08.F90 \
        iscatterv_f08.F90 \
        isend_f08.F90 \
        isendrecv_f08.F90 \
        isendrecv_replace_f08.F90 \
        issend_f08.F90 \
        is_thread_main_f08.F90 \
        lookup_name_f08.F90 \
        mprobe_f08.F90 \
        mrecv_f08.F90 \
        neighbor_allgather_f08.F90 \
        neighbor_allgather_init_f08.F90 \
        neighbor_allgatherv_f08.F90 \
        neighbor_allgatherv_init_f08.F90 \
        neighbor_alltoall_f08.F90 \
        neighbor_alltoall_init_f08.F90 \
        neighbor_alltoallv_f08.F90 \
        neighbor_alltoallv_init_f08.F90 \
        neighbor_alltoallw_f08.F90 \
        neighbor_alltoallw_init_f08.F90 \
        op_commutative_f08.F90 \
        op_create_f08.F90 \
        open_port_f08.F90 \
        op_free_f08.F90 \
        pack_external_f08.F90 \
        pack_external_size_f08.F90 \
        pack_f08.F90 \
        pack_size_f08.F90 \
        parrived_f08.F90 \
        pcontrol_f08.F90 \
        pready_f08.F90 \
        pready_range_f08.F90 \
        pready_list_f08.F90 \
        precv_init_f08.F90 \
        probe_f08.F90 \
        psend_init_f08.F90 \
        publish_name_f08.F90 \
        put_f08.F90 \
        query_thread_f08.F90 \
        raccumulate_f08.F90 \
        recv_f08.F90 \
        recv_init_f08.F90 \
        reduce_f08.F90 \
        reduce_init_f08.F90 \
        reduce_local_f08.F90 \
        reduce_scatter_f08.F90 \
        reduce_scatter_init_f08.F90 \
        reduce_scatter_block_f08.F90 \
        reduce_scatter_block_init_f08.F90 \
        register_datarep_f08.F90 \
        request_free_f08.F90 \
        request_get_status_f08.F90 \
        rget_f08.F90 \
        rget_accumulate_f08.F90 \
        rput_f08.F90 \
        rsend_f08.F90 \
        rsend_init_f08.F90 \
        scan_f08.F90 \
        scan_init_f08.F90 \
        scatter_f08.F90 \
        scatter_init_f08.F90 \
        scatterv_f08.F90 \
        scatterv_init_f08.F90 \
        send_init_f08.F90 \
        sendrecv_f08.F90 \
        sendrecv_replace_f08.F90 \
        session_call_errhandler_f08.F90\
        session_create_errhandler_f08.F90\
        session_get_errhandler_f08.F90\
	session_get_info_f08.F90 \
	session_get_nth_pset_f08.F90 \
	session_get_num_psets_f08.F90 \
	session_get_pset_info_f08.F90 \
	session_init_f08.F90 \
	session_finalize_f08.F90 \
        session_set_errhandler_f08.F90\
        ssend_f08.F90 \
        ssend_init_f08.F90 \
        startall_f08.F90 \
        start_f08.F90 \
        status_f082f_f08.F90 \
        status_f2f08_f08.F90 \
        status_set_cancelled_f08.F90 \
        status_set_elements_f08.F90 \
        status_set_elements_x_f08.F90 \
        testall_f08.F90 \
        testany_f08.F90 \
        test_cancelled_f08.F90 \
        test_f08.F90 \
        testsome_f08.F90 \
        topo_test_f08.F90 \
        type_commit_f08.F90 \
        type_contiguous_f08.F90 \
        type_create_darray_f08.F90 \
        type_create_f90_complex_f08.F90 \
        type_create_f90_integer_f08.F90 \
        type_create_f90_real_f08.F90 \
        type_create_hindexed_f08.F90 \
        type_create_hvector_f08.F90 \
        type_create_indexed_block_f08.F90 \
        type_create_hindexed_block_f08.F90 \
        type_create_keyval_f08.F90 \
        type_create_resized_f08.F90 \
        type_create_struct_f08.F90 \
        type_create_subarray_f08.F90 \
        type_delete_attr_f08.F90 \
        type_dup_f08.F90 \
        type_free_f08.F90 \
        type_free_keyval_f08.F90 \
        type_get_attr_f08.F90 \
        type_get_contents_f08.F90 \
        type_get_envelope_f08.F90 \
        type_get_extent_f08.F90 \
        type_get_extent_x_f08.F90 \
        type_get_name_f08.F90 \
        type_get_true_extent_f08.F90 \
        type_get_true_extent_x_f08.F90 \
        type_indexed_f08.F90 \
        type_match_size_f08.F90 \
        type_set_attr_f08.F90 \
        type_set_name_f08.F90 \
        type_size_f08.F90 \
        type_size_x_f08.F90 \
        type_vector_f08.F90 \
        unpack_external_f08.F90 \
        unpack_f08.F90 \
        unpublish_name_f08.F90 \
        waitall_f08.F90 \
        waitany_f08.F90 \
        wait_f08.F90 \
        waitsome_f08.F90 \
        win_allocate_f08.F90 \
        win_allocate_shared_f08.F90 \
        win_attach_f08.F90 \
        win_call_errhandler_f08.F90 \
        win_complete_f08.F90 \
        win_create_dynamic_f08.F90 \
        win_create_errhandler_f08.F90 \
        win_create_f08.F90 \
        win_create_keyval_f08.F90 \
        win_delete_attr_f08.F90 \
        win_detach_f08.F90 \
        win_fence_f08.F90 \
        win_flush_f08.F90 \
        win_flush_all_f08.F90 \
        win_flush_local_f08.F90 \
        win_flush_local_all_f08.F90 \
        win_free_f08.F90 \
        win_free_keyval_f08.F90 \
        win_get_attr_f08.F90 \
        win_get_errhandler_f08.F90 \
        win_get_group_f08.F90 \
        win_get_info_f08.F90 \
        win_get_name_f08.F90 \
        win_lock_f08.F90 \
        win_lock_all_f08.F90 \
        win_post_f08.F90 \
        win_set_attr_f08.F90 \
        win_set_errhandler_f08.F90 \
        win_set_info_f08.F90 \
        win_set_name_f08.F90 \
        win_shared_query_f08.F90 \
        win_start_f08.F90 \
        win_sync_f08.F90 \
        win_test_f08.F90 \
        win_unlock_f08.F90 \
        win_unlock_all_f08.F90 \
        win_wait_f08.F90 \
        $(mpi_api_generated_files)

# JMS Somehow this variable substitution isn't quite working, and I
# don't have time to figure it out.  So just wholesale copy the file
# list.  :-(
#pmpi_api_files = $(mpi_api_files:%=p%)

lib@OMPI_LIBMPI_NAME@_usempif08_la_SOURCES = \
        $(mpi_api_files) \
        mpi-f08.F90

# These are generated; do not ship them
# nodist_lib@OMPI_LIBMPI_NAME@_usempif08_la_SOURCES =

if BUILD_FORTRAN_SIZEOF
SIZEOF_H = sizeof_f08.h
lib@OMPI_LIBMPI_NAME@_usempif08_la_SOURCES += \
        sizeof_f08.h \
        sizeof_f08.f90 \
        psizeof_f08.f90
endif

lib@OMPI_LIBMPI_NAME@_usempif08_la_FCFLAGS = \
        $(AM_FCFLAGS) \
        -DOMPI_BUILD_MPI_PROFILING=0

#
# Include the mpi_f08-based MPI extensions in libmpi_usempif08, too.
#
# Also include the one .c file that we need in this library -- because
# we zero out CPPFLAGS and AM_CPPFLAGS in this Makefile.am, we have to
# compile that .c file in a separate directory / Makefile.
#

lib@OMPI_LIBMPI_NAME@_usempif08_la_LIBADD = \
        lib@OMPI_LIBMPI_NAME@_usempif08_pmpi.la \
        $(OMPI_MPIEXT_USEMPIF08_LIBS) \
        $(top_builddir)/ompi/mpi/fortran/mpif-h/lib@OMPI_LIBMPI_NAME@_mpifh.la \
        $(top_builddir)/ompi/lib@OMPI_LIBMPI_NAME@.la \
        mod/libusempif08_internal_modules.la \
        base/libusempif08_ccode.la
lib@OMPI_LIBMPI_NAME@_usempif08_la_DEPENDENCIES = $(module_sentinel_files)
lib@OMPI_LIBMPI_NAME@_usempif08_la_LDFLAGS = -version-info $(libmpi_usempif08_so_version)

#
# Automake doesn't do Fortran dependency analysis, so must list them
# manually here.  Bummer!
#

mpi_api_lo_files = $(mpi_api_files:.F90=.lo)

$(mpi_api_lo_files): bindings/libforce_usempif08_internal_bindings_to_be_built.la

mpi-f08.lo: $(module_sentinel_files) $(SIZEOF_H)
mpi-f08.F90: $(SIZEOF_H)

#
# Profiling interface
#

pmpi_api_files = \
        profile_abort_f08.F90 \
        profile_accumulate_f08.F90 \
        profile_add_error_class_f08.F90 \
        profile_add_error_code_f08.F90 \
        profile_add_error_string_f08.F90 \
        profile_paint_add_f08.F90 \
        profile_paint_diff_f08.F90 \
        profile_allgather_f08.F90 \
        profile_allgather_init_f08.F90 \
        profile_allgatherv_f08.F90 \
        profile_allgatherv_init_f08.F90 \
        profile_alloc_mem_f08.F90 \
        profile_allreduce_f08.F90 \
        profile_alltoall_f08.F90 \
        profile_alltoall_init_f08.F90 \
        profile_alltoallv_f08.F90 \
        profile_alltoallv_init_f08.F90 \
        profile_alltoallw_f08.F90 \
        profile_alltoallw_init_f08.F90 \
        profile_barrier_f08.F90 \
        profile_barrier_init_f08.F90 \
        profile_bcast_f08.F90 \
        profile_bcast_init_f08.F90 \
        profile_bsend_f08.F90 \
        profile_bsend_init_f08.F90 \
        profile_buffer_attach_f08.F90 \
        profile_buffer_detach_f08.F90 \
        profile_cancel_f08.F90 \
        profile_cart_coords_f08.F90 \
        profile_cart_create_f08.F90 \
        profile_cartdim_get_f08.F90 \
        profile_cart_get_f08.F90 \
        profile_cart_map_f08.F90 \
        profile_cart_rank_f08.F90 \
        profile_cart_shift_f08.F90 \
        profile_cart_sub_f08.F90 \
        profile_close_port_f08.F90 \
        profile_comm_accept_f08.F90 \
        profile_comm_call_errhandler_f08.F90 \
        profile_comm_compare_f08.F90 \
        profile_comm_connect_f08.F90 \
        profile_comm_create_errhandler_f08.F90 \
        profile_comm_create_f08.F90 \
        profile_comm_create_group_f08.F90 \
        profile_comm_create_keyval_f08.F90 \
        profile_comm_delete_attr_f08.F90 \
        profile_comm_disconnect_f08.F90 \
        profile_comm_dup_f08.F90 \
        profile_comm_dup_with_info_f08.F90 \
        profile_comm_idup_f08.F90 \
        profile_comm_idup_with_info_f08.F90 \
        profile_comm_free_f08.F90 \
        profile_comm_free_keyval_f08.F90 \
        profile_comm_get_attr_f08.F90 \
        profile_comm_get_errhandler_f08.F90 \
        profile_comm_get_info_f08.F90 \
        profile_comm_get_name_f08.F90 \
        profile_comm_get_parent_f08.F90 \
        profile_comm_group_f08.F90 \
        profile_comm_join_f08.F90 \
        profile_comm_rank_f08.F90 \
        profile_comm_remote_group_f08.F90 \
        profile_comm_remote_size_f08.F90 \
        profile_comm_set_attr_f08.F90 \
        profile_comm_set_errhandler_f08.F90 \
        profile_comm_set_info_f08.F90 \
        profile_comm_set_name_f08.F90 \
        profile_comm_size_f08.F90 \
        profile_comm_spawn_f08.F90 \
        profile_comm_spawn_multiple_f08.F90 \
        profile_comm_split_f08.F90 \
        profile_comm_split_type_f08.F90 \
        profile_comm_test_inter_f08.F90 \
        profile_compare_and_swap_f08.F90 \
        profile_dims_create_f08.F90 \
        profile_dist_graph_create_adjacent_f08.F90 \
        profile_dist_graph_create_f08.F90 \
        profile_dist_graph_neighbors_count_f08.F90 \
        profile_dist_graph_neighbors_f08.F90 \
        profile_errhandler_free_f08.F90 \
        profile_error_class_f08.F90 \
        profile_error_string_f08.F90 \
        profile_exscan_f08.F90 \
        profile_exscan_init_f08.F90 \
        profile_f_sync_reg_f08.F90 \
        profile_fetch_and_op_f08.F90 \
        profile_file_call_errhandler_f08.F90 \
        profile_file_close_f08.F90 \
        profile_file_create_errhandler_f08.F90 \
        profile_file_delete_f08.F90 \
        profile_file_get_amode_f08.F90 \
        profile_file_get_atomicity_f08.F90 \
        profile_file_get_byte_offset_f08.F90 \
        profile_file_get_errhandler_f08.F90 \
        profile_file_get_group_f08.F90 \
        profile_file_get_info_f08.F90 \
        profile_file_get_position_f08.F90 \
        profile_file_get_position_shared_f08.F90 \
        profile_file_get_size_f08.F90 \
        profile_file_get_type_extent_f08.F90 \
        profile_file_get_view_f08.F90 \
        profile_file_iread_at_f08.F90 \
        profile_file_iread_f08.F90 \
        profile_file_iread_at_all_f08.F90 \
        profile_file_iread_all_f08.F90 \
        profile_file_iread_shared_f08.F90 \
        profile_file_iwrite_at_f08.F90 \
        profile_file_iwrite_f08.F90 \
        profile_file_iwrite_at_all_f08.F90 \
        profile_file_iwrite_all_f08.F90 \
        profile_file_iwrite_shared_f08.F90 \
        profile_file_open_f08.F90 \
        profile_file_preallocate_f08.F90 \
        profile_file_read_all_begin_f08.F90 \
        profile_file_read_all_end_f08.F90 \
        profile_file_read_all_f08.F90 \
        profile_file_read_at_all_begin_f08.F90 \
        profile_file_read_at_all_end_f08.F90 \
        profile_file_read_at_all_f08.F90 \
        profile_file_read_at_f08.F90 \
        profile_file_read_f08.F90 \
        profile_file_read_ordered_begin_f08.F90 \
        profile_file_read_ordered_end_f08.F90 \
        profile_file_read_ordered_f08.F90 \
        profile_file_read_shared_f08.F90 \
        profile_file_seek_f08.F90 \
        profile_file_seek_shared_f08.F90 \
        profile_file_set_atomicity_f08.F90 \
        profile_file_set_errhandler_f08.F90 \
        profile_file_set_info_f08.F90 \
        profile_file_set_size_f08.F90 \
        profile_file_set_view_f08.F90 \
        profile_file_sync_f08.F90 \
        profile_file_write_all_begin_f08.F90 \
        profile_file_write_all_end_f08.F90 \
        profile_file_write_all_f08.F90 \
        profile_file_write_at_all_begin_f08.F90 \
        profile_file_write_at_all_end_f08.F90 \
        profile_file_write_at_all_f08.F90 \
        profile_file_write_at_f08.F90 \
        profile_file_write_f08.F90 \
        profile_file_write_ordered_begin_f08.F90 \
        profile_file_write_ordered_end_f08.F90 \
        profile_file_write_ordered_f08.F90 \
        profile_file_write_shared_f08.F90 \
        profile_finalized_f08.F90 \
        profile_finalize_f08.F90 \
        profile_free_mem_f08.F90 \
        profile_gather_f08.F90 \
        profile_gather_init_f08.F90 \
        profile_gatherv_f08.F90 \
        profile_gatherv_init_f08.F90 \
        profile_get_accumulate_f08.F90 \
        profile_get_address_f08.F90 \
        profile_get_count_f08.F90 \
        profile_get_elements_f08.F90 \
        profile_get_elements_x_f08.F90 \
        profile_get_f08.F90 \
        profile_get_library_version_f08.F90 \
        profile_get_processor_name_f08.F90 \
        profile_get_version_f08.F90 \
        profile_graph_create_f08.F90 \
        profile_graphdims_get_f08.F90 \
        profile_graph_get_f08.F90 \
        profile_graph_map_f08.F90 \
        profile_graph_neighbors_count_f08.F90 \
        profile_graph_neighbors_f08.F90 \
        profile_grequest_complete_f08.F90 \
        profile_grequest_start_f08.F90 \
        profile_group_compare_f08.F90 \
        profile_group_difference_f08.F90 \
        profile_group_excl_f08.F90 \
        profile_group_free_f08.F90 \
        profile_group_incl_f08.F90 \
        profile_group_intersection_f08.F90 \
        profile_group_range_excl_f08.F90 \
        profile_group_range_incl_f08.F90 \
        profile_group_rank_f08.F90 \
        profile_group_size_f08.F90 \
        profile_group_translate_ranks_f08.F90 \
        profile_group_union_f08.F90 \
        profile_iallgather_f08.F90 \
        profile_iallgatherv_f08.F90 \
        profile_iallreduce_f08.F90 \
        profile_ialltoall_f08.F90 \
        profile_ialltoallv_f08.F90 \
        profile_ialltoallw_f08.F90 \
        profile_ibarrier_f08.F90 \
        profile_ibcast_f08.F90 \
        profile_ibsend_f08.F90 \
        profile_igather_f08.F90 \
        profile_igatherv_f08.F90 \
        profile_iexscan_f08.F90 \
        profile_improbe_f08.F90 \
        profile_imrecv_f08.F90 \
        profile_ineighbor_allgather_f08.F90 \
        profile_ineighbor_allgatherv_f08.F90 \
        profile_ineighbor_alltoall_f08.F90 \
        profile_ineighbor_alltoallv_f08.F90 \
        profile_ineighbor_alltoallw_f08.F90 \
        profile_info_create_f08.F90 \
        profile_info_create_env_f08.F90 \
        profile_info_delete_f08.F90 \
        profile_info_dup_f08.F90 \
        profile_info_free_f08.F90 \
        profile_info_get_f08.F90 \
        profile_info_get_nkeys_f08.F90 \
        profile_info_get_nthkey_f08.F90 \
        profile_info_get_string_f08.F90 \
        profile_info_get_valuelen_f08.F90 \
        profile_info_set_f08.F90 \
        profile_init_f08.F90 \
        profile_initialized_f08.F90 \
        profile_init_thread_f08.F90 \
        profile_intercomm_create_f08.F90 \
        profile_intercomm_merge_f08.F90 \
        profile_iprobe_f08.F90 \
        profile_irecv_f08.F90 \
        profile_ireduce_f08.F90 \
        profile_ireduce_scatter_f08.F90 \
        profile_ireduce_scatter_block_f08.F90 \
        profile_irsend_f08.F90 \
        profile_iscan_f08.F90 \
        profile_iscatter_f08.F90 \
        profile_iscatterv_f08.F90 \
        profile_isend_f08.F90 \
        profile_isendrecv_f08.F90 \
        profile_isendrecv_replace_f08.F90 \
        profile_issend_f08.F90 \
        profile_is_thread_main_f08.F90 \
        profile_lookup_name_f08.F90 \
        profile_mprobe_f08.F90 \
        profile_mrecv_f08.F90 \
        profile_neighbor_allgather_f08.F90 \
        profile_neighbor_allgather_init_f08.F90 \
        profile_neighbor_allgatherv_f08.F90 \
        profile_neighbor_allgatherv_init_f08.F90 \
        profile_neighbor_alltoall_f08.F90 \
        profile_neighbor_alltoall_init_f08.F90 \
        profile_neighbor_alltoallv_f08.F90 \
        profile_neighbor_alltoallv_init_f08.F90 \
        profile_neighbor_alltoallw_f08.F90 \
        profile_neighbor_alltoallw_init_f08.F90 \
        profile_op_commutative_f08.F90 \
        profile_op_create_f08.F90 \
        profile_open_port_f08.F90 \
        profile_op_free_f08.F90 \
        profile_pack_external_f08.F90 \
        profile_pack_external_size_f08.F90 \
        profile_pack_f08.F90 \
        profile_pack_size_f08.F90 \
        profile_pcontrol_f08.F90 \
        profile_probe_f08.F90 \
        profile_publish_name_f08.F90 \
        profile_put_f08.F90 \
        profile_query_thread_f08.F90 \
        profile_raccumulate_f08.F90 \
        profile_recv_f08.F90 \
        profile_recv_init_f08.F90 \
        profile_reduce_f08.F90 \
        profile_reduce_init_f08.F90 \
        profile_reduce_local_f08.F90 \
        profile_reduce_scatter_f08.F90 \
        profile_reduce_scatter_init_f08.F90 \
        profile_reduce_scatter_block_f08.F90 \
        profile_reduce_scatter_block_init_f08.F90 \
        profile_register_datarep_f08.F90 \
        profile_request_free_f08.F90 \
        profile_request_get_status_f08.F90 \
        profile_rget_f08.F90 \
        profile_rget_accumulate_f08.F90 \
        profile_rput_f08.F90 \
        profile_rsend_f08.F90 \
        profile_rsend_init_f08.F90 \
        profile_scan_f08.F90 \
        profile_scan_init_f08.F90 \
        profile_scatter_f08.F90 \
        profile_scatter_init_f08.F90 \
        profile_scatterv_f08.F90 \
        profile_scatterv_init_f08.F90 \
        profile_send_f08_generated.F90 \
        profile_send_init_f08.F90 \
        profile_sendrecv_f08.F90 \
        profile_sendrecv_replace_f08.F90 \
        profile_session_create_errhandler_f08.F90\
        profile_session_get_errhandler_f08.F90\
        profile_session_get_info_f08.F90 \
        profile_session_get_nth_pset_f08.F90 \
        profile_session_get_num_psets_f08.F90 \
        profile_session_get_pset_info_f08.F90 \
        profile_session_init_f08.F90 \
        profile_session_finalize_f08.F90 \
        profile_session_set_errhandler_f08.F90\
        profile_ssend_f08.F90 \
        profile_ssend_init_f08.F90 \
        profile_startall_f08.F90 \
        profile_start_f08.F90 \
        profile_status_f082f_f08.F90 \
        profile_status_f2f08_f08.F90 \
        profile_status_set_cancelled_f08.F90 \
        profile_status_set_elements_f08.F90 \
        profile_status_set_elements_x_f08.F90 \
        profile_testall_f08.F90 \
        profile_testany_f08.F90 \
        profile_test_cancelled_f08.F90 \
        profile_test_f08.F90 \
        profile_testsome_f08.F90 \
        profile_topo_test_f08.F90 \
        profile_type_commit_f08.F90 \
        profile_type_contiguous_f08.F90 \
        profile_type_create_darray_f08.F90 \
        profile_type_create_f90_complex_f08.F90 \
        profile_type_create_f90_integer_f08.F90 \
        profile_type_create_f90_real_f08.F90 \
        profile_type_create_hindexed_f08.F90 \
        profile_type_create_hvector_f08.F90 \
        profile_type_create_indexed_block_f08.F90 \
        profile_type_create_hindexed_block_f08.F90 \
        profile_type_create_keyval_f08.F90 \
        profile_type_create_resized_f08.F90 \
        profile_type_create_struct_f08.F90 \
        profile_type_create_subarray_f08.F90 \
        profile_type_delete_attr_f08.F90 \
        profile_type_dup_f08.F90 \
        profile_type_free_f08.F90 \
        profile_type_free_keyval_f08.F90 \
        profile_type_get_attr_f08.F90 \
        profile_type_get_contents_f08.F90 \
        profile_type_get_envelope_f08.F90 \
        profile_type_get_extent_f08.F90 \
        profile_type_get_extent_x_f08.F90 \
        profile_type_get_name_f08.F90 \
        profile_type_get_true_extent_f08.F90 \
        profile_type_get_true_extent_x_f08.F90 \
        profile_type_indexed_f08.F90 \
        profile_type_match_size_f08.F90 \
        profile_type_set_attr_f08.F90 \
        profile_type_set_name_f08.F90 \
        profile_type_size_f08.F90 \
        profile_type_size_x_f08.F90 \
        profile_type_vector_f08.F90 \
        profile_unpack_external_f08.F90 \
        profile_unpack_f08.F90 \
        profile_unpublish_name_f08.F90 \
        profile_waitall_f08.F90 \
        profile_waitany_f08.F90 \
        profile_wait_f08.F90 \
        profile_waitsome_f08.F90 \
        profile_win_allocate_f08.F90 \
        profile_win_allocate_shared_f08.F90 \
        profile_win_attach_f08.F90 \
        profile_win_call_errhandler_f08.F90 \
        profile_win_complete_f08.F90 \
        profile_win_create_dynamic_f08.F90 \
        profile_win_create_errhandler_f08.F90 \
        profile_win_create_f08.F90 \
        profile_win_create_keyval_f08.F90 \
        profile_win_delete_attr_f08.F90 \
        profile_win_detach_f08.F90 \
        profile_win_fence_f08.F90 \
        profile_win_flush_f08.F90 \
        profile_win_flush_all_f08.F90 \
        profile_win_flush_local_f08.F90 \
        profile_win_flush_local_all_f08.F90 \
        profile_win_free_f08.F90 \
        profile_win_free_keyval_f08.F90 \
        profile_win_get_attr_f08.F90 \
        profile_win_get_errhandler_f08.F90 \
        profile_win_get_group_f08.F90 \
        profile_win_get_info_f08.F90 \
        profile_win_get_name_f08.F90 \
        profile_win_lock_f08.F90 \
        profile_win_lock_all_f08.F90 \
        profile_win_post_f08.F90 \
        profile_win_set_attr_f08.F90 \
        profile_win_set_errhandler_f08.F90 \
        profile_win_set_info_f08.F90 \
        profile_win_set_name_f08.F90 \
        profile_win_shared_query_f08.F90 \
        profile_win_start_f08.F90 \
        profile_win_sync_f08.F90 \
        profile_win_test_f08.F90 \
        profile_win_unlock_f08.F90 \
        profile_win_unlock_all_f08.F90 \
        profile_win_wait_f08.F90

pmpi_api_lo_files = $(pmpi_api_files:.F90=.lo)

$(pmpi_api_lo_files): bindings/libforce_usempif08_internal_bindings_to_be_built.la

lib@OMPI_LIBMPI_NAME@_usempif08_pmpi_la_SOURCES = \
        $(pmpi_api_files)

lib@OMPI_LIBMPI_NAME@_usempif08_pmpi_la_FCFLAGS = \
        $(AM_FCFLAGS) \
        -DOMPI_BUILD_MPI_PROFILING=1
#
# Sym link in the real sources
#
$(lib@OMPI_LIBMPI_NAME@_usempif08_pmpi_la_SOURCES):
	$(OMPI_V_LN_S) if test ! -r $@ ; then \
		pname=`echo $@ | cut -b '9-'` ; \
		$(LN_S) $(top_srcdir)/ompi/mpi/fortran/use-mpi-f08/$$pname $@ ; \
	fi

# These files were created by targets above

MAINTAINERCLEANFILES = $(lib@OMPI_LIBMPI_NAME@_usempif08_pmpi_la_SOURCES)

#
# Automake doesn't do Fortran dependency analysis, so must list them
# manually here.  Bummer!
#


#
# Generate the Fortran bindings and C wrapper functions for bindings with a
# *.in template.
#

if OMPI_GENERATE_BINDINGS
%_generated.F90: %.in generate_bindings.py
	$(PYTHON) $(srcdir)/generate_bindings.py fortran $< > $@

MAINTAINERCLEANFILES += $(mpi_api_generated_files)
endif

###########################################################################

# Install the generated .mod files.  Unfortunately, each F90 compiler
# may generate different filenames, so we have to use a glob.  :-(

install-exec-hook:
	@ for file in `ls *.mod`; do \
	  echo $(INSTALL) $$file $(DESTDIR)$(libdir); \
	  $(INSTALL) $$file $(DESTDIR)$(libdir); \
	done

uninstall-local:
	@ for file in `ls *.mod`; do \
	  echo rm -f $(DESTDIR)$(libdir)/$$file; \
	  rm -f $(DESTDIR)$(libdir)/$$file; \
	done

endif
