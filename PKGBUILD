pkgname="auto-commit"
pkgver=2.0.0
pkgrel=1
pkgdesc="AI-powered Git commit message generator - supports OpenAI, DeepSeek, and Gemini"
arch=("x86_64" "aarch64")
license=("MIT")
url='https://github.com/clearclown/auto-commit'
makedepends=("git" "cargo")
source=("git+https://github.com/clearclown/auto-commit.git")
sha512sums=("SKIP")

pkgver() {
  cd "$pkgname"
  git describe --tags | sed 's/^v//;s/\([^-]*-g\)/r\1/;s/-/./g'
}

build() {
  cd "$pkgname"
  cargo build --release --locked
}

package() {
  cd "$pkgname"
  install -Dm755 target/release/auto-commit "$pkgdir/usr/bin/auto-commit"
  install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
