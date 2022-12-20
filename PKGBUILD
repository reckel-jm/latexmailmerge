# Maintainer: Jan Martin Reckel <jm.reckel@t-online.de>
pkgname=latexmailmerge-bin
pkgver=0.1.0
pkgrel=1
epoch=
pkgdesc="A small command line tool which converts a CSV file into LaTeX commands for mail merges"
arch=('x86_64')
url="https://github.com/reckel-jm/latexmailmerge/"
license=('GPL3')
groups=()
provides=("latexmailmerge")
source=("https://github.com/reckel-jm/latexmailmerge/releases/download/v$pkgver/latexmailmerge-linux_x64")
md5sums=('e2dc68ac9b2f716e443db79874228595')

package() {
	mv latexmailmerge-linux_x64 latexmailmerge
	mkdir -p $pkgdir/usr/bin/
	install -D latexmailmerge $pkgdir/usr/bin/latexmailmerge
}
