# MCA_btl_rsm_CONFIG([action-if-can-compile],
#                    [action-if-cant-compile])
# ------------------------------------------------
AC_DEFUN([MCA_opal_btl_rsm_CONFIG],[
    AC_CONFIG_FILES([opal/mca/btl/rsm/Makefile])

    $1

    OPAL_SUMMARY_ADD([[Transports]],[[Shared memory/copy in+copy out written in Rust]],[$1],[yes])
])dnl
