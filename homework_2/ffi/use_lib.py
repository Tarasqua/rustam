from ctypes import CDLL, Structure, byref, c_uint64, cdll


class Params(Structure):
    _fields_ = [
        ("left", c_uint64),
        ("right", c_uint64),
    ]


def add_numbers_c(lib: CDLL, left: int, right: int):
    result = lib.add_numbers_c(a := 2, b := 3)
    print(f"{a} + {b} = {result}")


def add_numbers_c_with_params(lib: CDLL, params: Params):
    result = lib.add_numbers_c_with_params(byref(params))  # putting as a reference
    print(f"{params.left} + {params.right} = {result}")


if __name__ == "__main__":
    lib = cdll.LoadLibrary("./target/debug/libsynt_ex_lib.so")

    add_numbers_c(lib, 2, 3)
    add_numbers_c_with_params(lib, Params(5, 6))
