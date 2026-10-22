> [!WARNING]
> **OpenLogi 仍在积极开发中**，尚未稳定 —— 功能与配置仍可能变动。点个 **Star** ⭐ 并 **Watch** 👀 本仓库，在新版本发布时获得通知。

<p align="center">
    <img src="https://assets.openlogi.org/brand/openlogi-icon.png" width="138" alt="OpenLogi"/>
</p>

<h1 align="center">OpenLogi</h1>
<p align="center"><strong>⚡️ 本地优先的 Logitech Options+ 平替 <br/>通过 HID++ 与 UVC 协议解锁 Logitech 鼠标、键盘与摄像头的完整能力</strong></p>


<div align="center">
    <a href="https://github.com/AprilNEA/OpenLogi/releases/latest" target="_blank">
    <img alt="GitHub release" src="https://img.shields.io/github/v/release/AprilNEA/OpenLogi?style=flat"></a>
    <a href="https://github.com/AprilNEA/OpenLogi/releases" target="_blank">
    <img alt="GitHub downloads" src="https://img.shields.io/github/downloads/AprilNEA/OpenLogi/total.svg?style=flat"></a>
    <a href="https://github.com/AprilNEA/OpenLogi/commits" target="_blank">
    <img alt="GitHub commit" src="https://img.shields.io/github/commit-activity/m/AprilNEA/OpenLogi?style=flat"></a>
    <a href="#许可证">
    <img alt="License" src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat"></a>
    <img alt="Hits" src="https://hits.aprilnea.com/hits?url=https://github.com/aprilnea/openlogi">
</div>

<div align="center">
    <a href="https://x.com/AprilNEA" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/follow-AprilNEA-black?style=flat&logo=x"></a>
    <a href="https://t.me/+VDtkR5OSAT04NzVh" target="_blank">
    <img alt="telegram" src="https://img.shields.io/badge/chat-telegram-blueviolet?style=flat&logo=Telegram"></a>
</div>

<div align="center">
    <a href="../README.md">
    <img alt="English" src="https://img.shields.io/badge/English-lightgrey?style=flat"></a>
    <a href="README.zh-CN.md">
    <img alt="简体中文" src="https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-blue?style=flat"></a>
    <a href="README.ja.md">
    <img alt="日本語" src="https://img.shields.io/badge/%E6%97%A5%E6%9C%AC%E8%AA%9E-lightgrey?style=flat"></a>
    <a href="README.de.md">
    <img alt="Deutsch" src="https://img.shields.io/badge/Deutsch-lightgrey?style=flat"></a>
    <a href="README.fr.md">
    <img alt="Français" src="https://img.shields.io/badge/Fran%C3%A7ais-lightgrey?style=flat"></a>
    <a href="README.ko.md">
    <img alt="한국어" src="assets/badge-lang-ko.svg"></a>
    <a href="README.ru.md">
    <img alt="Русский" src="https://img.shields.io/badge/%D0%A0%D1%83%D1%81%D1%81%D0%BA%D0%B8%D0%B9-lightgrey?style=flat"></a>
    <a href="README.es.md">
    <img alt="Español" src="https://img.shields.io/badge/Espa%C3%B1ol-lightgrey?style=flat"></a>
    <a href="README.pt-BR.md">
    <img alt="Português" src="https://img.shields.io/badge/Portugu%C3%AAs-lightgrey?style=flat"></a>
    <a href="README.tr.md">
    <img alt="Türkçe" src="https://img.shields.io/badge/T%C3%BCrk%C3%A7e-lightgrey?style=flat"></a>
</div>

<p align="center">
    <a href="https://trendshift.io/repositories/42303" target="_blank">
    <img src="https://trendshift.io/api/badge/repositories/42303" alt="AprilNEA%2FOpenLogi | Trendshift" width="250" height="55"/></a>
    <a href="https://www.producthunt.com/products/openlogi?embed=true&amp;utm_source=badge-featured&amp;utm_medium=badge&amp;utm_campaign=badge-openlogi" target="_blank" rel="noopener noreferrer">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=openlogi&amp;theme=dark&amp;period=daily">
        <source media="(prefers-color-scheme: light)" srcset="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=openlogi&amp;theme=light&amp;period=daily">
        <img alt="OpenLogi - A local-first alternative to Logitech Options+ | Product Hunt" width="250" height="54" src="https://api.producthunt.com/widgets/embed-image/v1/top-post-badge.svg?post_id=openlogi&amp;theme=light&amp;period=daily">
    </picture></a>
</p>

> **被 Options+ 折腾够了？试试 OpenLogi。**

支持 macOS、Linux 和 Windows。

---

## 超越 Options+

OpenLogi 能做、而 Options+ 做不到的事：

- **轻量**：原生 Rust + GPUI。
- **支持 Linux**：Linux 是 OpenLogi 的一等平台。
- **在受支持的按键上使用手势**：给受支持的控件分配手势动作，也可以完全关闭手势。
- **纯文本配置**：所有设置都在一个 TOML 文件里，用什么方式在多台机器之间同步都可以。
- **可脚本化**：GUI 之外还有完整的 CLI，提供 JSON [自动化 API](USAGE.md#automation-api)（DPI、SmartShift、Fn 锁定、回报率）和硬件诊断。

## 功能

<p align="center">
    <img src="https://assets.openlogi.org/screenshots/macos/light/devices-grid.png" width="860" alt="OpenLogi 设备列表：六台 Logitech 设备，包括鼠标、键盘和 Litra 补光灯">
</p>

### 设备

- 支持 Logi Bolt 和 Unifying 接收器、蓝牙以及有线连接，显示电池电量和充电状态
- 电量低时在屏幕底部提示，每次放电只提示一次，不需要系统通知权限
- 键盘切换 Caps Lock 时在屏幕上显示提示；Windows 和 Linux 上还包括 Num Lock 和 Scroll Lock

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/easy-switch.png" width="400" alt="Easy-Switch 卡片，列出三台已配对的电脑"></td>
        <td width="50%"><b>Easy-Switch</b><ul><li>查看每个通道配对的电脑、系统和连接方式</li><li>设备允许时可以重命名通道</li></ul></td>
    </tr>
</table>

### 按键与动作

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/buttons.png" width="400" alt="MX Master 4 的按键映射，已打开手势键检查器"></td>
        <td width="50%"><b>重新映射任意按键</b><ul><li>内置动作、自定义快捷键、文本输入或多步工作流</li><li>单击和长按可以分别设置动作；按住型组合键在松开按键时释放，适合按键说话</li><li>在中键、后退和前进键、DPI 或模式切换键、手势键和触觉面板上使用手势</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/actions-ring.png" width="400" alt="Actions Ring 编辑器：光标周围的八个槽位"></td>
        <td width="50%"><b>Actions Ring</b><ul><li>光标周围一圈八个动作</li><li>每个应用可以有自己的布局</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/keys.png" width="400" alt="MX Keys S 上三个按键分别绑定到启动台、锁定屏幕和截屏"></td>
        <td width="50%"><b>键盘按键</b><ul><li>像鼠标按键一样绑定媒体、表情、截屏等专用键</li><li>可以把单个按键恢复为设备默认，或重置整排按键</li></ul></td>
    </tr>
</table>

按应用配置随前台应用自动切换。用 `[apps]` 别名可以让同一份配置跨平台跟随一个应用。

### 指针与滚动

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/dpi.png" width="400" alt="指针调节卡片，带 DPI 滑块"></td>
        <td width="50%"><b>指针</b><ul><li>DPI 预设，支持循环切换和切到指定预设</li><li>指针速度和回报率</li><li>精准模式：一个按键把鼠标切到最低速度，再按一次切回</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/smartshift.png" width="400" alt="SmartShift 卡片：滚轮模式、灵敏度和滚动力度"></td>
        <td width="50%"><b>SmartShift</b><ul><li>滚轮模式和灵敏度</li><li>SmartShift Enhanced 鼠标可调滚动力度</li><li>永久棘轮开关</li></ul></td>
    </tr>
    <tr>
        <td colspan="2"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/scrolling.png" width="820" alt="滚动卡片：反转方向、滚轮分辨率和平滑滚轮"><br><b>滚动</b><ul><li>按设备反转滚动方向，设备支持时原生实现，否则由软件实现</li><li>滚轮分辨率和平滑滚轮</li><li>在没有拇指滚轮的鼠标上，按住后退或前进键再滚动主滚轮即可横向滚动</li></ul></td>
    </tr>
</table>

### 键盘与灯光

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/keys-backlight.png" width="400" alt="MX Keys S 的按键和背光卡片"></td>
        <td width="50%"><b>键盘</b><ul><li>Fn Lock；多平台键盘保持当前系统的布局</li><li>背光档位、自动亮度和省电模式</li><li>调整背光时在屏幕上显示档位</li><li>受支持键盘的静态 RGB 灯光</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/litra.png" width="400" alt="Litra Glow 灯光面板：亮度和色温"></td>
        <td width="50%"><b>Litra 补光灯</b><ul><li>开关、亮度和色温</li><li>可以跟随摄像头自动开灯</li><li>在灯上直接做的调整也会保存</li></ul></td>
    </tr>
</table>

### 摄像头

- 支持任何 Logitech UVC 摄像头（Brio、StreamCam、C920 系列等），即插即用
- 实时预览只在查看时占用摄像头，离开预览即释放，指示灯随之熄灭
- 变焦、平移、俯仰、对焦、曝光、白平衡等全部 UVC 控制直接写入硬件，Meet、Zoom、OBS 都能看到变化
- 内置「默认」「直播」「视频通话」三个配置档，另可保存自定义快照，按摄像头分别保存

能用哪些功能取决于设备报告的能力，应用只显示设备支持的控件。

### 平台支持

| 功能 | macOS | Windows | Linux |
| --- | --- | --- | --- |
| 鼠标按键映射和手势 | ✓ | ✓ | ✓ |
| 键盘 F 键映射 | ✓ | ✓ | — |
| 按应用配置 | ✓ | ✓ | X11、wlroots（sway、Hyprland），以及装有[配套扩展](../crates/openlogi-hook/gnome-shell-extension/README.md)的 GNOME |
| 鼠标配置跟随指针所在窗口 | ✓ | ✓ | 仅 X11 |
| 媒体键 | 原生 | 原生 | D-Bus MPRIS |
| 窗口管理动作（切换桌面、调度中心等） | 原生 | 原生对应功能 | GNOME、KDE（KWin）、Hyprland；部分 macOS 专属动作无效果 |
| 菜单栏或托盘图标 | ✓ | ✓ | — |
| 应用内更新 | Sparkle | 内置 | 内置 |

## 安装

> [!IMPORTANT]
> 先退出 **Logi Options+**：两者会争夺 HID++ 访问权，同一个接收器同时只能由一方占用。

### macOS

需要 macOS 13 或更高版本。

从[最新 release](https://github.com/AprilNEA/OpenLogi/releases/latest) 下载已签名、已公证的 `.dmg`，把 `OpenLogi.app` 拖入 `/Applications`。在「设置 → 更新」中开启检查更新后，应用会通过 Sparkle 自动更新。

也可以通过 [Homebrew](https://brew.sh) 安装：

```sh
brew install --cask openlogi
```

官方 Homebrew cask 是默认的安装途径。如果想改用 `aprilnea/tap` 直接跟踪 GitHub 最新 release：

```sh
brew tap aprilnea/tap
brew install --cask aprilnea/tap/openlogi@latest
```

`openlogi@latest` 由 OpenLogi 的发布工作流维护，可能比官方 cask 的自动更新先到。`openlogi` 和 `openlogi@latest` 只装一个。

### Linux

通过 HTTPS 把安装脚本下载到文件，检查后再运行，不要通过管道直接交给 shell：

```sh
curl --proto '=https' --proto-redir '=https' --tlsv1.2 \
  --fail --location --silent --show-error \
  --retry 3 --retry-connrefused \
  --output openlogi-install.sh \
  https://raw.githubusercontent.com/AprilNEA/OpenLogi/master/packaging/linux/install.sh
less openlogi-install.sh
sh openlogi-install.sh
rm openlogi-install.sh
```

脚本会按包管理器和架构选择对应的 `.deb`、`.rpm` 或 `.pkg.tar.zst`，验证 minisign 签名和校验和，安装 udev 规则，并把 agent 作为用户服务启动。先通过发行版安装 `minisign`，然后以普通用户运行脚本，不要用 `sudo`。安装包支持 `x86_64` 和 `arm64`，要求 GLIBC 2.35 或更高版本。

每个 release 还附带 Flatpak 包，仓库也提供 NixOS 模块。这两种方式、固定版本安装、源码安装、无 systemd 的发行版以及 Hyprland 和 Omarchy 说明，见 [INSTALL-linux.md](INSTALL-linux.md)。

### Windows

每个 release 都附带签名的便携式 `.zip` 和按用户安装的 `.msi`（x86_64 与 arm64）。两者都包含 GUI（`OpenLogi.exe`）和负责全部设备 I/O 的后台 agent（`openlogi-agent.exe`）。使用便携式 zip 时，这两个文件必须放在同一目录，否则 GUI 无法连接 agent。

Windows 版已在 Windows 11 实机上完成端到端验证，硬件为一把有线键盘和一只使用 Unifying 接收器的鼠标，验证范围包括 MSI 的安装、原位升级和卸载。它比 macOS 版更新，遇到问题时请[提交反馈](https://github.com/AprilNEA/OpenLogi/issues)。agent 会显示系统托盘图标（「显示主窗口」和「退出」），关闭主窗口后仍能打开应用；在「设置 → 通用」中可以关闭该图标。

从源码构建见 [DEVELOPMENT.md](DEVELOPMENT.md)。

## 使用（CLI）

见 [USAGE.md](USAGE.md)。

## 配置

设置保存为纯文本 TOML；保存时会保留以符号链接方式管理的配置文件。见 [CONFIGURATION.md](CONFIGURATION.md)。

## 开发

见 [DEVELOPMENT.md](DEVELOPMENT.md)，其中包括 [macOS 输入钩子安全](DEVELOPMENT.md#macos-input-hook-safety)。

## 致谢

- **Windows、摄像头与 i18n**：[@davidbudnick](https://github.com/davidbudnick) —— 键盘 RGB 支持、Windows 支持、Logitech 摄像头支持
- **Linux 移植**：[@cserby](https://github.com/cserby) —— Linux 支持
- [Solaar](https://github.com/pwr-Solaar/Solaar)，作者 [@pwr](https://github.com/pwr) —— 开源 HID++ 实现
- [Mouser](https://github.com/TomBadash/Mouser)，作者 [@TomBadash](https://github.com/TomBadash) —— 本地、无需账号的 Options+ 替代品

## 许可证

本仓库代码可选择以下任一许可证：

- Apache License 2.0（[LICENSE-APACHE](../LICENSE-APACHE)）
- MIT 许可证（[LICENSE-MIT](../LICENSE-MIT)）

### 第三方代码

`crates/openlogi-hidpp` 是 [`hidpp`](https://crates.io/crates/hidpp)（作者 [@lus](https://github.com/lus)）的 vendored fork，采用 0BSD 许可证。

### Logo 与品牌资产

感谢 [@kubai087](https://github.com/kubai087) 为 OpenLogi 设计的 Logo，该 Logo —— 即 [`design/`](../design/) 下的品牌资产 —— © 2026 AprilNEA 保留所有权利，不在上述 MIT/Apache 许可范围内，许可证详见 [`design/LICENSE`](../design/LICENSE)。
Fork 代码并不授予 OpenLogi 名称、Logo 或图标的使用权，未经事先书面许可，请勿用它们代表你自己的项目、Fork 或分发版本。

---

**与 Logitech 无关联。** 「Logitech」、「MX Master」与「Options+」是 Logitech International S.A. 的商标。

## 仓库活跃度

![Repobeats analytics image](https://repobeats.axiom.co/api/embed/4a0b576a03e9d528ad31ccf4797a1286c045d021.svg "Repobeats analytics image")
