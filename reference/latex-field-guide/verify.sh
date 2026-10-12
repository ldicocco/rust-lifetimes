#!/usr/bin/env bash
# Compiles every example with rustc (edition 2021) and checks its expectation:
#   ok_*.rs   must compile
#   fail_*.rs must be rejected by the compiler
# Usage: ./verify.sh           (run from this directory; needs rustc)
set -u
pass=0; bad=0
tmp=$(mktemp -d)
for f in ex/*.rs; do
  n=$(basename "$f" .rs)
  if rustc --edition 2021 -A dead_code -A unused -o "$tmp/$n" "$f" >/dev/null 2>&1; then rc=0; else rc=1; fi
  case "$n" in
    ok_*)   [ $rc -eq 0 ] && r=PASS || r=FAIL ;;
    fail_*) [ $rc -ne 0 ] && r=PASS || r=FAIL ;;
    *)      r=SKIP ;;
  esac
  printf '%-34s %s\n' "$n" "$r"
  if [ "$r" = PASS ]; then pass=$((pass+1)); elif [ "$r" = FAIL ]; then bad=$((bad+1)); fi
done
rm -rf "$tmp"
echo "pass=$pass fail=$bad"
[ "$bad" -eq 0 ]
