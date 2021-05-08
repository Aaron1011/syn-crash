set -xe

rm -rf my_incr
cp lib.rs temp_lib.rs
rustc --crate-type lib -C opt-level=3 temp_lib.rs -C incremental=my_incr
sed -i '28 i\\' temp_lib.rs
rustc --crate-type lib -C opt-level=3 temp_lib.rs -C incremental=my_incr
