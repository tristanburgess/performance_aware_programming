@echo off

call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" amd64

mkdir build
pushd build
cl /Zi /EHsc /O2 ..\examples\p2_waste\add.cpp
popd