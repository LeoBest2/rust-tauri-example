## 环境
- ubuntu
- rust
- nodejs 、pnpm 
- 系统依赖

```bash
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

## 开发

```bash
cd 当前目录
# 安装依赖库
pnpm install
# 开发
pnpm tauri dev
# 构建
pnpm tauri build
```

## 效果图
![](https://img-blog.csdnimg.cn/974a5ba66bb4430ea90a4e55dcfe106b.png#pic_center)
