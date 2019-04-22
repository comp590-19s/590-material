from cffi import FFI
ffi = FFI()
ffi.cdef("void print_hello_from_rust();");

rust = ffi.dlopen("./libhello.so")

rust.print_hello_from_rust()
