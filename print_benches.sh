#!/usr/bin/env bash
set -euo pipefail

for q in {01..22}; do
  seq_ms=$(jq -r '.mean.point_estimate/1e6|round' \
    target/criterion/tpch_${q}/SF1_sequential/new/estimates.json 2>/dev/null \
    || echo n/a)
  par_ms=$(jq -r '.mean.point_estimate/1e6|round' \
    target/criterion/tpch_${q}/SF1_parallel/new/estimates.json 2>/dev/null \
    || echo n/a)
  printf "Q%2s: sequential=%4s ms  parallel=%4s ms\n" "$q" "$seq_ms" "$par_ms"
done
