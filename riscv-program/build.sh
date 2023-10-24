rm -rf build
cmake -Bbuild -H. || exit
make -C build