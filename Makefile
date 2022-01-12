PROJECT=ticheck

.PHONY: all clean test

default: main buildsucc

server-admin-check: server_check buildsucc

main:
	@>&2  rm -rf bin && mkdir bin && cd bin
	# @>&2  rm -rf ./*.docx
	@>&2  cargo build
	@>&2  mv ./target/debug/tihc ./bin/
	# @>&2  mv /tmp/*ticheck_*
	@echo Start building tool TiHC successfully!

buildsucc:
	@echo Build tool TiCheck successfully!

all: dev server benchkv

grafana-image-render-check:
	yum  -y install libXcomposite libXdamage libXtst cups libXScrnSaver \
	pango atk adwaita-cursor-theme adwaita-icon-theme at at-spi2-atk at-spi2-core cairo-gobject \
	colord-libs dconf desktop-file-utils ed emacs-filesystem gdk-pixbuf2 glib-networking gnutls \
	gsettings-desktop-schemas gtk-update-icon-cache gtk3 hicolor-icon-theme jasper-libs json-glib \
	libappindicator-gtk3 libdbusmenu libdbusmenu-gtk3 libepoxy liberation-fonts liberation-narrow-fonts \
	liberation-sans-fonts liberation-serif-fonts libgusb libindicator-gtk3 libmodman libproxy libsoup \
	libwayland-cursor libwayland-egl libxkbcommon m4 mailx nettle patch psmisc redhat-lsb-core \
	redhat-lsb-submod-security rest spax time trousers xdg-utils xkeyboard-config alsa-lib
	@echo All of library function that a depends on has been downloaded successfully!

dev: 
	@>&2 cargo fmt
	@>&2 cargo test
	@>&2 echo "Great!, all tests passed."

clean:

