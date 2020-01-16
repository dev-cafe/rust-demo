from cffi import FFI

_ffi = FFI()
_ffi.cdef("""
    double pi_approximation(int num_points);
""")

# hardcoded here out of laziness
_lib = _ffi.dlopen("../../target/release/libpi.so")

pi_approximation = _lib.pi_approximation
