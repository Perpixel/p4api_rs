#!/usr/bin/env sh

mkdir ./extern

#https://cdist2.perforce.com/perforce/r23.1/bin.ntx64/p4api_vs2022_static.zip

#wget https://cdist2.perforce.com/perforce/r23.1/bin.linux26x86_64/p4api.tgz -O ./tmp/p4api.tgz
wget https://cdist2.perforce.com/perforce/r23.1/bin.linux26x86_64/p4api-glibc2.3-openssl3.tgz -O ./tmp/p4api.tgz
tar -zxf ./tmp/p4api.tgz -C ./extern/

wget https://www.openssl.org/source/openssl-3.0.10.tar.gz -O ./tmp/openssl.tgz
tar -zxf ./tmp/openssl.tgz -C ./extern/
