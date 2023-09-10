#!/usr/bin/env sh

echo "Build test p4api wrapper..."
clang -fdeclspec -fno-inline-functions -x c++ -std=c++14 -c -D OS_LINUX -I./extern/p4api_linux/include/ -o ./out/p4api_wrapper.o ./p4api_wrapper/p4api_wrapper.cpp
echo "Link..."
ar r ./lib/libp4api_wrapper.a ./out/p4api_wrapper.o
