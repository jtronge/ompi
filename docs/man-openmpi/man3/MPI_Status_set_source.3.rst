.. _mpi_status_set_source:


MPI_Status_set_source
========================

.. include_body

:ref:`MPI_Status_set_source` |mdash| Sets the MPI_SOURCE field on *status*.


SYNTAX
------


C Syntax
^^^^^^^^

.. code-block:: c

   #include <mpi.h>

   int MPI_Status_set_source(MPI_Status *status, int source)


Fortran Syntax
^^^^^^^^^^^^^^

.. code-block:: fortran

   USE MPI
   ! or the older form: INCLUDE 'mpif.h'
   MPI_STATUS_SET_SOURCE(STATUS, SOURCE, IERROR)
   	INTEGER	STATUS(MPI_STATUS_SIZE), IERROR
   	INTEGER SOURCE


Fortran 2008 Syntax
^^^^^^^^^^^^^^^^^^^

.. code-block:: fortran

   USE mpi_f08
   MPI_Status_set_source(status, source, ierror)
   	TYPE(MPI_Status), INTENT(INOUT) :: status
   	INTEGER, INTENT(IN) :: source
   	INTEGER, OPTIONAL, INTENT(OUT) :: ierror


INPUT/OUTPUT PARAMETER
----------------------
* ``status``: Status with which to associate source rank (status).

INPUT PARAMETER
---------------
* ``source``: rank to set in the MPI_SOURCE field (integer).

OUTPUT PARAMETER
----------------
* ``ierror``: Fortran only: Error status (integer).

DESCRIPTION
-----------

Set the MPI_SOURCE field in the status object to the provided source argument.


ERRORS
------

.. include:: ./ERRORS.rst
