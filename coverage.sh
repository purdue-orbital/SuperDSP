line=$(cargo +nightly llvm-cov | tail -1 | awk '{print $4 $7 $10}');

if [ "$line" = "100.00%100.00%100.00%" ]; then
	exit 0;
else
	exit 1;
fi
