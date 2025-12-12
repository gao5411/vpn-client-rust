# 构建 Linux x86_64（musl 静态）
cargo build --release --target x86_64-unknown-linux-musl

# 构建 OpenWrt ARMv7（如树莓派）
cargo build --release --target armv7-unknown-linux-musleabihf

# 构建 OpenWrt MIPS（老旧路由器）
cargo build --release --target mipsel-unknown-linux-musl
