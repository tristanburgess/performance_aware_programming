@echo off

call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" amd64

mkdir build
pushd build
if "%1" == "clang" (
    clang++ -target x86_64-pc-windows-gnu ..\examples\p4_simd\simd_ipc_sum.cpp -o simd_ipc_sum.exe
) else (
    cl /Zi /EHsc /O2 ..\examples\p4_simd\simd_ipc_sum.cpp
)
popd