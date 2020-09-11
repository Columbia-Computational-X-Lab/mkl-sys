#!/bin/bash
set -uex

bindgen \
  --use-core \
  --with-derive-{default,eq,hash,ord} \
  cblas.h \
  > src/blas.rs
