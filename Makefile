env:
	@echo "export LD_LIBRARY_PATH=~/Mantra:${LD_LIBRARY_PATH}"
	@echo "export PATH=~/Mantra:${PATH}"


install-libwasvm:
	wget -P /usr/lib https://github.com/CosmWasm/wasmvm/releases/download/v1.5.4/libwasmvm.x86_64.so
