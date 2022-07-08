exe_name=rcat
install_path=~/.local/bin/

help:
	@echo -e "install     -   Build and install ${exe_name}."
	@echo -e "uninstall   -   Remove ${exe_name} files."
	@echo -e "clean       -   Remove target directory."

install:
	cargo build --release
	cp -fv ./target/release/${exe_name} ${install_path}

uninstall:
	rm -fv ${install_path}${exe_name}

clean:
	cargo clean