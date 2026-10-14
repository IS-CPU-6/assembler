# ./compare.sh a.log b.log [context]
CTX="${3:-3}"

awk -v ctx="$CTX" -v f1="$1" -v f2="$2" '
  function header() {
    printf "\033[31m--- %s\033[0m\n", f1
    printf "\033[32m+++ %s\033[0m\n", f2
  }

  function show_ctx(m,    i, from, skip) {
    header()
    skip = m - ctx - 1
    if (skip > 0) printf "(%d lines)\n", skip
    from = (m - ctx > 1) ? m - ctx : 1
    for (i = from; i < m; i++) printf "%6d  %s\n", i, a[i]
  }

  function del(n, s) { printf "\033[31m%6d -%s\033[0m\n", n, s }
  function add(n, s) { printf "\033[32m%6d +%s\033[0m\n", n, s }

  NR == FNR {
    a[NR] = $0
    n1 = NR
    next
  }

  {
    n2 = FNR
    if (FNR > n1) {
      show_ctx(FNR)
      del(FNR, "(EOF)")
      add(FNR, $0)
      mismatch = 1
      line1 = n1 + 1
      line2 = FNR
      exit
    }
    if (a[FNR] != $0) {
      show_ctx(FNR)
      del(FNR, a[FNR])
      add(FNR, $0)
      mismatch = 1
      line1 = FNR
      line2 = FNR
      exit
    }
  }

  END {
    if (!mismatch && n2 < n1) {
      show_ctx(n2 + 1)
      del(n2 + 1, a[n2 + 1])
      add(n2 + 1, "(EOF)")
      mismatch = 1
      line1 = n2 + 1
      line2 = n2
    }

    if (mismatch) {
      # exit 後の END では FNR が途中なので、2つ目の総行数は読み切って数える
      while ((getline tmp < f2) > 0) total2++
      rem1 = (n1 > line1) ? n1 - line1 : 0
      rem2 = (total2 > line2) ? total2 - line2 : 0
      if (rem1 > 0 || rem2 > 0)
        printf "(\033[31m%d\033[0m / \033[32m%d\033[0m lines)\n", rem1, rem2
    }
    exit mismatch
  }
' "$1" "$2"