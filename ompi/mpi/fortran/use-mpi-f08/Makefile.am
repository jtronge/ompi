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

profile/psizeof_f08.f90: $(top_builddir)/config.status
profile/psizeof_f08.f90: $(sizeof_pl)
profile/psizeof_f08.f90:
	$(OMPI_V_GEN) $(sizeof_pl) \
	    --impl=$@ --ierror=optional --pmpi \
	    --maxrank=$(OMPI_FORTRAN_MAX_ARRAY_RANK) \
	    --generate=$(OMPI_FORTRAN_BUILD_SIZEOF) \
	    --real2=$(OMPI_HAVE_FORTRAN_REAL2) \
	    --iso_real16=$(OMPI_FORTRAN_HAVE_ISO_FORTRAN_ENV_REAL16) \
	    --real16=$(OMPI_HAVE_FORTRAN_REAL16) \
	    --complex4=$(OMPI_HAVE_FORTRAN_COMPLEX4) \
	    --complex32=$(OMPI_HAVE_FORTRAN_COMPLEX32)

CLEANFILES += sizeof_f08.h sizeof_f08.f90 profile/psizeof_f08.f90

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
#pmpi_api_files = $(mpi_api_files:%=profile/p%)

lib@OMPI_LIBMPI_NAME@_usempif08_la_SOURCES = \
        $(mpi_api_files) \
        mpi-f08.F90

lib@OMPI_LIBMPI_NAME@_usempif08_la_FCFLAGS = \
        $(AM_FCFLAGS) \
        -DOMPI_BUILD_MPI_PROFILING=0

# These are generated; do not ship them
nodist_lib@OMPI_LIBMPI_NAME@_usempif08_la_SOURCES =

if BUILD_FORTRAN_SIZEOF
SIZEOF_H = sizeof_f08.h
nodist_lib@OMPI_LIBMPI_NAME@_usempif08_la_SOURCES += \
        sizeof_f08.h \
        sizeof_f08.f90 \
        profile/psizeof_f08.f90
endif

#
# Include the mpi_f08-based MPI extensions in libmpi_usempif08, too.
#
# Also include the one .c file that we need in this library -- because
# we zero out CPPFLAGS and AM_CPPFLAGS in this Makefile.am, we have to
# compile that .c file in a separate directory / Makefile.
#

lib@OMPI_LIBMPI_NAME@_usempif08_la_LIBADD = \
        profile/libmpi_usempif08_pmpi.la \
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

#
# Profiling interface
#

pmpi_api_files = \
        pabort_f08.F90 \
        paccumulate_f08.F90 \
        padd_error_class_f08.F90 \
        padd_error_code_f08.F90 \
        padd_error_string_f08.F90 \
        paint_add_f08.F90 \
        paint_diff_f08.F90 \
        pallgather_f08.F90 \
        pallgather_init_f08.F90 \
        pallgatherv_f08.F90 \
        pallgatherv_init_f08.F90 \
        palloc_mem_f08.F90 \
        pallreduce_f08.F90 \
        palltoall_f08.F90 \
        palltoall_init_f08.F90 \
        palltoallv_f08.F90 \
        palltoallv_init_f08.F90 \
        palltoallw_f08.F90 \
        palltoallw_init_f08.F90 \
        pbarrier_f08.F90 \
        pbarrier_init_f08.F90 \
        pbcast_f08.F90 \
        pbcast_init_f08.F90 \
        pbsend_f08.F90 \
        pbsend_init_f08.F90 \
        pbuffer_attach_f08.F90 \
        pbuffer_detach_f08.F90 \
        pcancel_f08.F90 \
        pcart_coords_f08.F90 \
        pcart_create_f08.F90 \
        pcartdim_get_f08.F90 \
        pcart_get_f08.F90 \
        pcart_map_f08.F90 \
        pcart_rank_f08.F90 \
        pcart_shift_f08.F90 \
        pcart_sub_f08.F90 \
        pclose_port_f08.F90 \
        pcomm_accept_f08.F90 \
        pcomm_call_errhandler_f08.F90 \
        pcomm_compare_f08.F90 \
        pcomm_connect_f08.F90 \
        pcomm_create_errhandler_f08.F90 \
        pcomm_create_f08.F90 \
        pcomm_create_group_f08.F90 \
        pcomm_create_keyval_f08.F90 \
        pcomm_delete_attr_f08.F90 \
        pcomm_disconnect_f08.F90 \
        pcomm_dup_f08.F90 \
        pcomm_dup_with_info_f08.F90 \
        pcomm_idup_f08.F90 \
        pcomm_idup_with_info_f08.F90 \
        pcomm_free_f08.F90 \
        pcomm_free_keyval_f08.F90 \
        pcomm_get_attr_f08.F90 \
        pcomm_get_errhandler_f08.F90 \
        pcomm_get_info_f08.F90 \
        pcomm_get_name_f08.F90 \
        pcomm_get_parent_f08.F90 \
        pcomm_group_f08.F90 \
        pcomm_join_f08.F90 \
        pcomm_rank_f08.F90 \
        pcomm_remote_group_f08.F90 \
        pcomm_remote_size_f08.F90 \
        pcomm_set_attr_f08.F90 \
        pcomm_set_errhandler_f08.F90 \
        pcomm_set_info_f08.F90 \
        pcomm_set_name_f08.F90 \
        pcomm_size_f08.F90 \
        pcomm_spawn_f08.F90 \
        pcomm_spawn_multiple_f08.F90 \
        pcomm_split_f08.F90 \
        pcomm_split_type_f08.F90 \
        pcomm_test_inter_f08.F90 \
        pcompare_and_swap_f08.F90 \
        pdims_create_f08.F90 \
        pdist_graph_create_adjacent_f08.F90 \
        pdist_graph_create_f08.F90 \
        pdist_graph_neighbors_count_f08.F90 \
        pdist_graph_neighbors_f08.F90 \
        perrhandler_free_f08.F90 \
        perror_class_f08.F90 \
        perror_string_f08.F90 \
        pexscan_f08.F90 \
        pexscan_init_f08.F90 \
        pf_sync_reg_f08.F90 \
        pfetch_and_op_f08.F90 \
        pfile_call_errhandler_f08.F90 \
        pfile_close_f08.F90 \
        pfile_create_errhandler_f08.F90 \
        pfile_delete_f08.F90 \
        pfile_get_amode_f08.F90 \
        pfile_get_atomicity_f08.F90 \
        pfile_get_byte_offset_f08.F90 \
        pfile_get_errhandler_f08.F90 \
        pfile_get_group_f08.F90 \
        pfile_get_info_f08.F90 \
        pfile_get_position_f08.F90 \
        pfile_get_position_shared_f08.F90 \
        pfile_get_size_f08.F90 \
        pfile_get_type_extent_f08.F90 \
        pfile_get_view_f08.F90 \
        pfile_iread_at_f08.F90 \
        pfile_iread_f08.F90 \
        pfile_iread_at_all_f08.F90 \
        pfile_iread_all_f08.F90 \
        pfile_iread_shared_f08.F90 \
        pfile_iwrite_at_f08.F90 \
        pfile_iwrite_f08.F90 \
        pfile_iwrite_at_all_f08.F90 \
        pfile_iwrite_all_f08.F90 \
        pfile_iwrite_shared_f08.F90 \
        pfile_open_f08.F90 \
        pfile_preallocate_f08.F90 \
        pfile_read_all_begin_f08.F90 \
        pfile_read_all_end_f08.F90 \
        pfile_read_all_f08.F90 \
        pfile_read_at_all_begin_f08.F90 \
        pfile_read_at_all_end_f08.F90 \
        pfile_read_at_all_f08.F90 \
        pfile_read_at_f08.F90 \
        pfile_read_f08.F90 \
        pfile_read_ordered_begin_f08.F90 \
        pfile_read_ordered_end_f08.F90 \
        pfile_read_ordered_f08.F90 \
        pfile_read_shared_f08.F90 \
        pfile_seek_f08.F90 \
        pfile_seek_shared_f08.F90 \
        pfile_set_atomicity_f08.F90 \
        pfile_set_errhandler_f08.F90 \
        pfile_set_info_f08.F90 \
        pfile_set_size_f08.F90 \
        pfile_set_view_f08.F90 \
        pfile_sync_f08.F90 \
        pfile_write_all_begin_f08.F90 \
        pfile_write_all_end_f08.F90 \
        pfile_write_all_f08.F90 \
        pfile_write_at_all_begin_f08.F90 \
        pfile_write_at_all_end_f08.F90 \
        pfile_write_at_all_f08.F90 \
        pfile_write_at_f08.F90 \
        pfile_write_f08.F90 \
        pfile_write_ordered_begin_f08.F90 \
        pfile_write_ordered_end_f08.F90 \
        pfile_write_ordered_f08.F90 \
        pfile_write_shared_f08.F90 \
        pfinalized_f08.F90 \
        pfinalize_f08.F90 \
        pfree_mem_f08.F90 \
        pgather_f08.F90 \
        pgather_init_f08.F90 \
        pgatherv_f08.F90 \
        pgatherv_init_f08.F90 \
        pget_accumulate_f08.F90 \
        pget_address_f08.F90 \
        pget_count_f08.F90 \
        pget_elements_f08.F90 \
        pget_elements_x_f08.F90 \
        pget_f08.F90 \
        pget_library_version_f08.F90 \
        pget_processor_name_f08.F90 \
        pget_version_f08.F90 \
        pgraph_create_f08.F90 \
        pgraphdims_get_f08.F90 \
        pgraph_get_f08.F90 \
        pgraph_map_f08.F90 \
        pgraph_neighbors_count_f08.F90 \
        pgraph_neighbors_f08.F90 \
        pgrequest_complete_f08.F90 \
        pgrequest_start_f08.F90 \
        pgroup_compare_f08.F90 \
        pgroup_difference_f08.F90 \
        pgroup_excl_f08.F90 \
        pgroup_free_f08.F90 \
        pgroup_incl_f08.F90 \
        pgroup_intersection_f08.F90 \
        pgroup_range_excl_f08.F90 \
        pgroup_range_incl_f08.F90 \
        pgroup_rank_f08.F90 \
        pgroup_size_f08.F90 \
        pgroup_translate_ranks_f08.F90 \
        pgroup_union_f08.F90 \
        piallgather_f08.F90 \
        piallgatherv_f08.F90 \
        piallreduce_f08.F90 \
        pialltoall_f08.F90 \
        pialltoallv_f08.F90 \
        pialltoallw_f08.F90 \
        pibarrier_f08.F90 \
        pibcast_f08.F90 \
        pibsend_f08.F90 \
        pigather_f08.F90 \
        pigatherv_f08.F90 \
        piexscan_f08.F90 \
        pimprobe_f08.F90 \
        pimrecv_f08.F90 \
        pineighbor_allgather_f08.F90 \
        pineighbor_allgatherv_f08.F90 \
        pineighbor_alltoall_f08.F90 \
        pineighbor_alltoallv_f08.F90 \
        pineighbor_alltoallw_f08.F90 \
        pinfo_create_f08.F90 \
        pinfo_create_env_f08.F90 \
        pinfo_delete_f08.F90 \
        pinfo_dup_f08.F90 \
        pinfo_free_f08.F90 \
        pinfo_get_f08.F90 \
        pinfo_get_nkeys_f08.F90 \
        pinfo_get_nthkey_f08.F90 \
        pinfo_get_string_f08.F90 \
        pinfo_get_valuelen_f08.F90 \
        pinfo_set_f08.F90 \
        pinit_f08.F90 \
        pinitialized_f08.F90 \
        pinit_thread_f08.F90 \
        pintercomm_create_f08.F90 \
        pintercomm_merge_f08.F90 \
        piprobe_f08.F90 \
        pirecv_f08.F90 \
        pireduce_f08.F90 \
        pireduce_scatter_f08.F90 \
        pireduce_scatter_block_f08.F90 \
        pirsend_f08.F90 \
        piscan_f08.F90 \
        piscatter_f08.F90 \
        piscatterv_f08.F90 \
        pisend_f08.F90 \
        pisendrecv_f08.F90 \
        pisendrecv_replace_f08.F90 \
        pissend_f08.F90 \
        pis_thread_main_f08.F90 \
        plookup_name_f08.F90 \
        pmprobe_f08.F90 \
        pmrecv_f08.F90 \
        pneighbor_allgather_f08.F90 \
        pneighbor_allgather_init_f08.F90 \
        pneighbor_allgatherv_f08.F90 \
        pneighbor_allgatherv_init_f08.F90 \
        pneighbor_alltoall_f08.F90 \
        pneighbor_alltoall_init_f08.F90 \
        pneighbor_alltoallv_f08.F90 \
        pneighbor_alltoallv_init_f08.F90 \
        pneighbor_alltoallw_f08.F90 \
        pneighbor_alltoallw_init_f08.F90 \
        pop_commutative_f08.F90 \
        pop_create_f08.F90 \
        popen_port_f08.F90 \
        pop_free_f08.F90 \
        ppack_external_f08.F90 \
        ppack_external_size_f08.F90 \
        ppack_f08.F90 \
        ppack_size_f08.F90 \
        ppcontrol_f08.F90 \
        pprobe_f08.F90 \
        ppublish_name_f08.F90 \
        pput_f08.F90 \
        pquery_thread_f08.F90 \
        praccumulate_f08.F90 \
        precv_f08.F90 \
        precv_init_f08.F90 \
        preduce_f08.F90 \
        preduce_init_f08.F90 \
        preduce_local_f08.F90 \
        preduce_scatter_f08.F90 \
        preduce_scatter_init_f08.F90 \
        preduce_scatter_block_f08.F90 \
        preduce_scatter_block_init_f08.F90 \
        pregister_datarep_f08.F90 \
        prequest_free_f08.F90 \
        prequest_get_status_f08.F90 \
        prget_f08.F90 \
        prget_accumulate_f08.F90 \
        prput_f08.F90 \
        prsend_f08.F90 \
        prsend_init_f08.F90 \
        pscan_f08.F90 \
        pscan_init_f08.F90 \
        pscatter_f08.F90 \
        pscatter_init_f08.F90 \
        pscatterv_f08.F90 \
        pscatterv_init_f08.F90 \
        psend_f08_generated.F90 \
        psend_init_f08.F90 \
        psendrecv_f08.F90 \
        psendrecv_replace_f08.F90 \
        psession_create_errhandler_f08.F90\
        psession_get_errhandler_f08.F90\
        psession_get_info_f08.F90 \
        psession_get_nth_pset_f08.F90 \
        psession_get_num_psets_f08.F90 \
        psession_get_pset_info_f08.F90 \
        psession_init_f08.F90 \
        psession_finalize_f08.F90 \
        psession_set_errhandler_f08.F90\
        pssend_f08.F90 \
        pssend_init_f08.F90 \
        pstartall_f08.F90 \
        pstart_f08.F90 \
        pstatus_f082f_f08.F90 \
        pstatus_f2f08_f08.F90 \
        pstatus_set_cancelled_f08.F90 \
        pstatus_set_elements_f08.F90 \
        pstatus_set_elements_x_f08.F90 \
        ptestall_f08.F90 \
        ptestany_f08.F90 \
        ptest_cancelled_f08.F90 \
        ptest_f08.F90 \
        ptestsome_f08.F90 \
        ptopo_test_f08.F90 \
        ptype_commit_f08.F90 \
        ptype_contiguous_f08.F90 \
        ptype_create_darray_f08.F90 \
        ptype_create_f90_complex_f08.F90 \
        ptype_create_f90_integer_f08.F90 \
        ptype_create_f90_real_f08.F90 \
        ptype_create_hindexed_f08.F90 \
        ptype_create_hvector_f08.F90 \
        ptype_create_indexed_block_f08.F90 \
        ptype_create_hindexed_block_f08.F90 \
        ptype_create_keyval_f08.F90 \
        ptype_create_resized_f08.F90 \
        ptype_create_struct_f08.F90 \
        ptype_create_subarray_f08.F90 \
        ptype_delete_attr_f08.F90 \
        ptype_dup_f08.F90 \
        ptype_free_f08.F90 \
        ptype_free_keyval_f08.F90 \
        ptype_get_attr_f08.F90 \
        ptype_get_contents_f08.F90 \
        ptype_get_envelope_f08.F90 \
        ptype_get_extent_f08.F90 \
        ptype_get_extent_x_f08.F90 \
        ptype_get_name_f08.F90 \
        ptype_get_true_extent_f08.F90 \
        ptype_get_true_extent_x_f08.F90 \
        ptype_indexed_f08.F90 \
        ptype_match_size_f08.F90 \
        ptype_set_attr_f08.F90 \
        ptype_set_name_f08.F90 \
        ptype_size_f08.F90 \
        ptype_size_x_f08.F90 \
        ptype_vector_f08.F90 \
        punpack_external_f08.F90 \
        punpack_f08.F90 \
        punpublish_name_f08.F90 \
        pwaitall_f08.F90 \
        pwaitany_f08.F90 \
        pwait_f08.F90 \
        pwaitsome_f08.F90 \
        pwin_allocate_f08.F90 \
        pwin_allocate_shared_f08.F90 \
        pwin_attach_f08.F90 \
        pwin_call_errhandler_f08.F90 \
        pwin_complete_f08.F90 \
        pwin_create_dynamic_f08.F90 \
        pwin_create_errhandler_f08.F90 \
        pwin_create_f08.F90 \
        pwin_create_keyval_f08.F90 \
        pwin_delete_attr_f08.F90 \
        pwin_detach_f08.F90 \
        pwin_fence_f08.F90 \
        pwin_flush_f08.F90 \
        pwin_flush_all_f08.F90 \
        pwin_flush_local_f08.F90 \
        pwin_flush_local_all_f08.F90 \
        pwin_free_f08.F90 \
        pwin_free_keyval_f08.F90 \
        pwin_get_attr_f08.F90 \
        pwin_get_errhandler_f08.F90 \
        pwin_get_group_f08.F90 \
        pwin_get_info_f08.F90 \
        pwin_get_name_f08.F90 \
        pwin_lock_f08.F90 \
        pwin_lock_all_f08.F90 \
        pwin_post_f08.F90 \
        pwin_set_attr_f08.F90 \
        pwin_set_errhandler_f08.F90 \
        pwin_set_info_f08.F90 \
        pwin_set_name_f08.F90 \
        pwin_shared_query_f08.F90 \
        pwin_start_f08.F90 \
        pwin_sync_f08.F90 \
        pwin_test_f08.F90 \
        pwin_unlock_f08.F90 \
        pwin_unlock_all_f08.F90 \
        pwin_wait_f08.F90

pmpi_api_lo_files = $(pmpi_api_files:.F90=.lo)

$(pmpi_api_lo_files): bindings/libforce_usempif08_internal_bindings_to_be_built.la

noinst_LTLIBRARIES = libmpi_usempif08_pmpi.la

nodist_libmpi_usempif08_pmpi_la_SOURCES = \
        $(pmpi_api_files)

nodist_libmpi_usempif08_pmpi_la_FCFLAGS = \
        $(AM_FCFLAGS) \
        -DOMPI_BUILD_MPI_PROFILING=1
#
# Sym link in the real sources
#
$(nodist_libmpi_usempif08_pmpi_la_SOURCES):
	$(OMPI_V_LN_S) if test ! -r $@ ; then \
		pname=`echo $@ | cut -b '2-'` ; \
		$(LN_S) $(top_srcdir)/ompi/mpi/fortran/use-mpi-f08/$$pname $@ ; \
	fi

# These files were created by targets above

MAINTAINERCLEANFILES = $(nodist_libmpi_usempif08_pmpi_la_SOURCES)

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
