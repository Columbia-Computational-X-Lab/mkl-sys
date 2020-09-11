#!/bin/bash
set -uex

bindgen \
  --use-core \
  --with-derive-{default,eq,hash,ord} \
  ci/cblas.h \
  > src/b.rs
