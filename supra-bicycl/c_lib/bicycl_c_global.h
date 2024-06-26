#ifndef RITUAL_GLOBAL_H
#define RITUAL_GLOBAL_H

// This header includes system headers and declares functions
// required by all regular headers of the library.

// for fixed size integer types
#include <stdint.h>

// placement new statements require this
#include <new>

// original C++ library includes generated by cpp_to_rust
#include "bicycl.hpp"

#ifdef _WIN32
    #define RITUAL_EXPORT __declspec(dllexport)
#else
    #define RITUAL_EXPORT
#endif


// Calls destructor of `T` class. This template function
// is necessary because it's not possible to use `x->~T()`
// syntax directly if `T` contains `::`.
template<typename T>
void ritual_call_destructor(T* x) {
    x->~T();
}


#endif // RITUAL_GLOBAL_H
