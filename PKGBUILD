# Maintainer: Praveen <praveen@local>
pkgname=anime4k-mpv-installer-git
pkgver=1.0.0
pkgrel=1
pkgdesc="Automated Anime4K GLSL shader and keybinding installer for mpv"
arch=('any')
url="https://github.com/Praveensenpai/anime4k-mpv-installer"
license=('MIT')
depends=('bash' 'mpv' 'unzip' 'python')
makedepends=('git')
provides=('anime4k-mpv-installer')
conflicts=('anime4k-mpv-installer')
source=("git+https://github.com/Praveensenpai/anime4k-mpv-installer.git")
sha256sums=('SKIP')

pkgver() {
  cd "$srcdir/${pkgname%-git}" 2>/dev/null || cd "$srcdir"
  git describe --long --tags 2>/dev/null | sed 's/\([^-]*-g\)/r\1/;s/-/./g' || echo "1.0.0"
}

package() {
  cd "$srcdir/${pkgname%-git}" 2>/dev/null || cd "$srcdir"
  install -Dm755 bin/anime4k-mpv-installer "$pkgdir/usr/bin/anime4k-mpv-installer"
}
