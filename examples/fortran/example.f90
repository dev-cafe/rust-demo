module pi_interface

    use, intrinsic :: iso_c_binding, only: c_double, c_int

    implicit none

    public pi_approximation

    private

    interface pi_approximation
        function pi_approximation(num_points) result(r) bind (c)
            import :: c_double, c_int
            integer(c_int), value, intent(in) :: num_points
            real(c_double) :: r
        end function
    end interface

end module


program example

    use :: pi_interface, only: pi_approximation

    implicit none

    print *, 'Hello from Fortran code!'
    print *, 'Rust code returned:', pi_approximation(1000000)

end program
