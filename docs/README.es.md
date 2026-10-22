> [!WARNING]
> **OpenLogi está en desarrollo activo** y todavía no es estable: las funciones y la configuración aún pueden cambiar. Dale una **Star** ⭐ al repositorio y ponlo en **Watch** 👀 para enterarte en cuanto salga una versión nueva.

<p align="center">
    <img src="https://assets.openlogi.org/brand/openlogi-icon.png" width="138" alt="OpenLogi"/>
</p>

<h1 align="center">OpenLogi</h1>
<p align="center"><strong>⚡️ Una alternativa nativa y local-first a Logitech Options+, escrita en Rust 🦀<br/>Aprovecha al máximo los ratones, teclados y webcams de Logitech mediante HID++ y UVC</strong></p>

<div align="center">
    <a href="https://github.com/AprilNEA/OpenLogi/releases/latest" target="_blank">
    <img alt="GitHub release" src="https://img.shields.io/github/v/release/AprilNEA/OpenLogi?style=flat"></a>
    <a href="https://github.com/AprilNEA/OpenLogi/releases" target="_blank">
    <img alt="GitHub downloads" src="https://img.shields.io/github/downloads/AprilNEA/OpenLogi/total.svg?style=flat"></a>
    <a href="https://github.com/AprilNEA/OpenLogi/commits" target="_blank">
    <img alt="GitHub commit" src="https://img.shields.io/github/commit-activity/m/AprilNEA/OpenLogi?style=flat"></a>
    <a href="#licencia">
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
    <img alt="简体中文" src="https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-lightgrey?style=flat"></a>
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
    <img alt="Español" src="https://img.shields.io/badge/Espa%C3%B1ol-blue?style=flat"></a>
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

> **¿Harto de Options+? Prueba OpenLogi.**

Funciona en macOS, Linux y Windows.

---

## Más allá de Options+

Cosas que hace OpenLogi y Options+ no:

- **No pesar.** Rust nativo + GPUI.
- **Funcionar en Linux.** En OpenLogi, Linux es una plataforma de primera.
- **Gestos en los botones compatibles.** Asigna acciones de gestos a los controles compatibles, o desactiva los gestos por completo.
- **Configuración en texto plano.** Todo cabe en un único fichero TOML que puedes sincronizar entre máquinas como te convenga.
- **Automatizar.** Una CLI de verdad junto a la interfaz gráfica: una [API de automatización](USAGE.md#automation-api) en JSON para DPI, SmartShift, Fn-lock y frecuencia de sondeo, además de diagnósticos de hardware.

## Funciones

<p align="center">
    <img src="https://assets.openlogi.org/screenshots/macos/light/devices-grid.png" width="860" alt="Lista de dispositivos de OpenLogi con seis dispositivos Logitech: ratones, un teclado y una luz Litra">
</p>

### Dispositivos

- Receptores Logi Bolt y Unifying, Bluetooth y conexiones por cable, con nivel de batería y estado de carga
- Avisos de batería baja en la parte inferior de la pantalla, una vez por descarga y sin permiso de notificaciones del sistema
- Un indicador en pantalla cuando un teclado activa o desactiva Caps Lock, y también Num Lock y Scroll Lock en Windows y Linux

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/easy-switch.png" width="400" alt="Tarjeta de Easy-Switch con tres ordenadores emparejados"></td>
        <td width="50%"><b>Easy-Switch</b><ul><li>Qué ordenador está emparejado en cada canal, con su sistema operativo y su conexión</li><li>Cambia el nombre de un canal en los dispositivos que lo permiten</li></ul></td>
    </tr>
</table>

### Botones y acciones

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/buttons.png" width="400" alt="Asignación de botones de un MX Master 4, con el inspector del botón de gestos abierto"></td>
        <td width="50%"><b>Reasigna cualquier botón</b><ul><li>Un catálogo de acciones integrado, atajos personalizados, texto escrito o flujos de varios pasos</li><li>Acciones distintas para pulsar y mantener, y acordes de push-to-talk que se sueltan con el botón</li><li>Gestos en el clic central, Atrás/Adelante, DPI/ModeShift, el botón de gestos y el panel háptico</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/actions-ring.png" width="400" alt="Editor del Actions Ring con ocho huecos alrededor del cursor"></td>
        <td width="50%"><b>Actions Ring</b><ul><li>Ocho acciones en un anillo alrededor del cursor</li><li>Su propia distribución para cada aplicación</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/keys.png" width="400" alt="MX Keys S con tres teclas asignadas a Launchpad, Bloquear pantalla y Captura de pantalla"></td>
        <td width="50%"><b>Teclas del teclado</b><ul><li>Asigna las teclas multimedia, de emoji, de captura de pantalla y otras teclas dedicadas igual que los botones del ratón</li><li>Devuelve una tecla a su valor predeterminado del dispositivo o restablece toda la fila</li></ul></td>
    </tr>
</table>

Los perfiles por aplicación cambian con la app que tengas en primer plano. Un alias en `[apps]` permite que un mismo perfil siga a una aplicación en distintas plataformas.

### Puntero y desplazamiento

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/dpi.png" width="400" alt="Tarjeta de ajuste del puntero con un control deslizante de DPI"></td>
        <td width="50%"><b>Puntero</b><ul><li>Preajustes de DPI con acciones para recorrerlos y fijar uno</li><li>Velocidad del puntero y frecuencia de sondeo</li><li>Modo de precisión: un botón pasa el ratón a su velocidad más lenta y lo devuelve</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/smartshift.png" width="400" alt="Tarjeta de SmartShift con modo de la rueda, sensibilidad y fuerza de desplazamiento"></td>
        <td width="50%"><b>SmartShift</b><ul><li>Modo de la rueda y sensibilidad</li><li>Fuerza de desplazamiento en ratones con SmartShift Enhanced</li><li>Un interruptor de trinquete permanente</li></ul></td>
    </tr>
    <tr>
        <td colspan="2"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/scrolling.png" width="820" alt="Tarjeta de desplazamiento con inversión, resolución de la rueda y rueda suave"><br><b>Desplazamiento</b><ul><li>Inversión del desplazamiento por dispositivo, de forma nativa o por software</li><li>Resolución de la rueda y desplazamiento suave con la rueda</li><li>Mantén Atrás o Adelante y gira la rueda para desplazarte en horizontal en ratones sin rueda lateral</li></ul></td>
    </tr>
</table>

### Teclado e iluminación

<table>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/keys-backlight.png" width="400" alt="Tarjetas de Teclas y Retroiluminación de un MX Keys S"></td>
        <td width="50%"><b>Teclado</b><ul><li>Fn Lock, y teclados multiplataforma que mantienen la distribución del sistema anfitrión</li><li>Nivel de retroiluminación, brillo automático y ahorro de batería</li><li>El nivel de retroiluminación se muestra en pantalla mientras cambia</li><li>RGB estático en los teclados compatibles</li></ul></td>
    </tr>
    <tr>
        <td width="50%"><img src="https://assets.openlogi.org/screenshots/macos/light/cards/litra.png" width="400" alt="Panel de iluminación de una Litra Glow con brillo y temperatura de color"></td>
        <td width="50%"><b>Luces Litra</b><ul><li>Encendido, brillo y temperatura de color</li><li>Se encienden con la cámara si quieres</li><li>Respetan los cambios hechos en la propia luz</li></ul></td>
    </tr>
</table>

### Webcams

- Cualquier webcam UVC de Logitech (Brio, StreamCam, la serie C920, …), sin configurar nada
- Vista previa en directo que solo ocupa la cámara mientras la miras; al salir, el LED se apaga
- Zoom, panorámica, inclinación, enfoque, exposición, balance de blancos y el resto de controles UVC, escritos en el hardware para que Meet, Zoom y OBS vean el cambio
- Perfiles integrados Predeterminado / Transmisión / Videollamada, más instantáneas propias, guardados por cámara

Las funciones disponibles dependen de lo que informe cada dispositivo; la aplicación solo muestra los controles que el dispositivo admite.

### Compatibilidad por plataforma

| Función | macOS | Windows | Linux |
| --- | --- | --- | --- |
| Reasignación de botones del ratón y gestos | ✓ | ✓ | ✓ |
| Reasignación de las teclas F del teclado | ✓ | ✓ | — |
| Perfiles por aplicación | ✓ | ✓ | X11, wlroots (sway, Hyprland), GNOME con la [extensión complementaria](../crates/openlogi-hook/gnome-shell-extension/README.md) |
| El perfil del ratón sigue a la ventana bajo el puntero | ✓ | ✓ | Solo X11 |
| Teclas multimedia | Nativo | Nativo | D-Bus MPRIS |
| Acciones del gestor de ventanas (escritorios, Mission Control, …) | Nativo | Equivalentes nativos | GNOME, KDE (KWin), Hyprland; algunas acciones exclusivas de macOS no hacen nada |
| Icono en la barra de menús / área de notificación | ✓ | ✓ | — |
| Actualizaciones desde la aplicación | Sparkle | Integradas | Integradas |

## Instalación

> [!IMPORTANT]
> Cierra antes **Logi Options+**: las dos aplicaciones se pelean por el acceso HID++ y un receptor solo puede pertenecer a una a la vez.

### macOS

Requiere macOS 13 o posterior.

Descarga el `.dmg` firmado y notarizado desde la [última release](https://github.com/AprilNEA/OpenLogi/releases/latest) y arrastra `OpenLogi.app` a `/Applications`. La aplicación se actualiza sola mediante Sparkle si activas la búsqueda de actualizaciones en Ajustes → Actualizaciones.

O instálalo con [Homebrew](https://brew.sh):

```sh
brew install --cask openlogi
```

El cask oficial de Homebrew es la vía de instalación por defecto. Si prefieres
seguir explícitamente la última release de GitHub desde `aprilnea/tap`:

```sh
brew tap aprilnea/tap
brew install --cask aprilnea/tap/openlogi@latest
```

De `openlogi@latest` se encarga el flujo de publicación de OpenLogi, y puede
actualizarse antes de que el cask oficial se ponga al día. Instala `openlogi` o
`openlogi@latest`, pero no los dos.

### Linux

Descarga el instalador por HTTPS, revísalo y luego ejecútalo. No lo pases
directamente a una shell con una tubería:

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

El script elige el `.deb`, `.rpm` o `.pkg.tar.zst` adecuado para tu gestor de paquetes y tu arquitectura, verifica su firma minisign y su suma de comprobación, instala las reglas de udev y arranca el agente como servicio de usuario. Instala antes `minisign` y ejecuta el script con tu usuario normal, no con `sudo`. Hay paquetes para `x86_64` y `arm64`, y necesitan GLIBC 2.35 o superior.

Cada release incluye también un paquete Flatpak, y el repositorio ofrece un módulo de NixOS. En [INSTALL-linux.md](INSTALL-linux.md) tienes ambos, la instalación de una versión fija o desde el código fuente, las distribuciones sin systemd y notas para Hyprland/Omarchy.

### Windows

Cada release incluye archivos `.zip` portables firmados e instaladores `.msi`
por usuario (x86_64 y arm64). Ambos traen la interfaz gráfica (`OpenLogi.exe`)
junto al agente en segundo plano (`openlogi-agent.exe`), que es quien gestiona
toda la E/S con los dispositivos. Si usas el zip portable, mantén los dos
ficheros juntos o la interfaz no tendrá a qué conectarse.

La compatibilidad con Windows se ha validado de principio a fin en Windows 11
con hardware real (un teclado por cable y un ratón con receptor Unifying),
incluidas la instalación, la actualización sobre la anterior y la desinstalación
del MSI. Es más reciente que la versión de macOS, así que
[avísanos](https://github.com/AprilNEA/OpenLogi/issues) si te encuentras alguna
aspereza. El agente muestra un icono en el área de notificación (Mostrar ventana
principal / Salir de OpenLogi) para que la aplicación siga a mano después de
cerrar la ventana principal; puedes desactivarlo en Ajustes → General.

Para compilar desde el código fuente, consulta [DEVELOPMENT.md](DEVELOPMENT.md).


## Uso (CLI)

Consulta [USAGE.md](USAGE.md)

## Configuración

Los ajustes se guardan en TOML plano; al guardar se respetan los ficheros de configuración enlazados mediante enlaces simbólicos. Consulta [CONFIGURATION.md](CONFIGURATION.md).

## Desarrollo

Consulta [DEVELOPMENT.md](DEVELOPMENT.md), incluida la [seguridad del hook de entrada en macOS](DEVELOPMENT.md#macos-input-hook-safety).

## Agradecimientos

- **Windows, cámaras e i18n** por [@davidbudnick](https://github.com/davidbudnick) — RGB del teclado, compatibilidad con Windows, compatibilidad con las webcams de Logitech
- **Port a Linux** por [@cserby](https://github.com/cserby) — compatibilidad con Linux
- [Solaar](https://github.com/pwr-Solaar/Solaar) de [@pwr](https://github.com/pwr) — implementación de HID++ de código abierto
- [Mouser](https://github.com/TomBadash/Mouser) de [@TomBadash](https://github.com/TomBadash) — un sustituto de Options+ local y sin cuentas

## Licencia

El código de este repositorio tiene licencia doble, a tu elección:

- Apache License, versión 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))
- Licencia MIT ([LICENSE-MIT](../LICENSE-MIT))

### Código de terceros

`crates/openlogi-hidpp` es un fork integrado de [`hidpp`](https://crates.io/crates/hidpp)
de [@lus](https://github.com/lus), con licencia 0BSD.

### Logo y recursos de marca

Gracias a [@kubai087](https://github.com/kubai087) por diseñar el logo de
OpenLogi. El logo y el icono de la aplicación (los recursos de marca que hay en
[`design/`](../design/)) son © 2026 AprilNEA, todos los derechos reservados, y no
están cubiertos por las licencias MIT/Apache anteriores; consulta
[`design/LICENSE`](../design/LICENSE). Hacer un fork del código no da ningún
derecho sobre el nombre, el logo ni el icono de OpenLogi; por favor, no los uses
para representar tus propios proyectos, forks o distribuciones sin permiso
previo por escrito.

---

**Sin afiliación con Logitech.** «Logitech», «MX Master» y «Options+» son marcas comerciales de Logitech International S.A.

## Actividad del repositorio

![Repobeats analytics image](https://repobeats.com/AprilNEA/OpenLogi "Repobeats analytics image")
