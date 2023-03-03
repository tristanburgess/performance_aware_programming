@echo off

call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" amd64

mkdir build
pushd build
if "%1" == "clang" (
    clang++ -target x86_64-pc-windows-gnu ..\hw\01_8086\01_inst_decode\decode.cpp -o decode.exe
) else (
    cl /Zi /EHsc /O2 ..\hw\01_8086\01_inst_decode\decode.cpp
)
popd